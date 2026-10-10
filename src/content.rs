use std::collections::HashMap;

use ascalon_asset::packfile::cntc::PackContent;
use bevy::app::{App, Plugin, Update};
use bevy::asset::{AssetServer, Assets, Handle, VisitAssetDependencies};
use bevy::ecs::resource::Resource;
use bevy::ecs::schedule::IntoScheduleConfigs as _;
use bevy::ecs::schedule::common_conditions::{not, resource_exists};
use bevy::ecs::system::{Commands, Res};
use bevy::ecs::world::{FromWorld, World};
use sha2::{Digest, Sha256};

use crate::asset::Packfile;

pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ContentHandles>().add_systems(
            Update,
            load_content
                .run_if(not(resource_exists::<ContentContext>))
                .run_if(content_dependencies_loaded),
        );
    }
}

#[derive(Resource, VisitAssetDependencies)]
pub struct ContentHandles(#[dependency] Vec<Handle<Packfile>>);

impl FromWorld for ContentHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self(
            [
                "1282830", "1282831", "1282832", "1282833", "1282834", "1282835", "1282836",
                "1282837", "1282838", "1282839", "1282840", "1282841", "1282842", "1282843",
                "1282844", "1282845", "1282846", "1282847", "1282848", "1282849", "1282850",
                "1282851", "1282852", "1282853", "1282854", "1282855", "1282856", "1282857",
                "1282858", "1282859", "1282860", "1282861",
            ]
            .into_iter()
            .map(|path| asset_server.load(path))
            .collect(),
        )
    }
}

pub fn content_dependencies_loaded(
    asset_server: Res<AssetServer>,
    handles: Res<ContentHandles>,
) -> bool {
    asset_server.are_dependencies_loaded(&*handles)
}

#[derive(Resource, Default)]
pub struct ContentContext {
    by_type:    HashMap<u32, Vec<*const u8>>,
    by_guid:    HashMap<Guid, *const u8>,
    by_data_id: HashMap<u32, *const u8>,
    by_name:    HashMap<u64, *const u8>,
}

unsafe impl Sync for ContentContext {}

unsafe impl Send for ContentContext {}

impl ContentContext {
    pub fn by_type<'a, T: ContentType + 'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.by_type
            .get(&T::ID)
            .into_iter()
            .flatten()
            .map(|&ptr| unsafe { (ptr as *const T).as_ref() }.unwrap())
    }

    pub fn by_guid<T: ContentType>(&self, guid: Guid) -> Option<&T> {
        let ptr = *self.by_guid.get(&guid)?;
        unsafe { (ptr as *const T).as_ref() }
    }

    pub fn by_data_id<T: ContentType>(&self, data_id: u32) -> Option<&T> {
        let ptr = *self.by_data_id.get(&(T::ID << 22 | data_id))?;
        unsafe { (ptr as *const T).as_ref() }
    }

    pub fn by_name<T: ContentType>(&self, name: &str) -> Option<&T> {
        let (namespace, name) = name.rsplit_once('.')?;

        let ptr = *self
            .by_name
            .get(&(mangle_name(namespace) << 30 | mangle_name(name)))?;
        unsafe { (ptr as *const T).as_ref() }
    }
}

pub fn load_content(
    mut commands: Commands,
    assets: Res<Assets<Packfile>>,
    handles: Res<ContentHandles>,
) {
    let content_all: Vec<_> = handles
        .0
        .iter()
        .map(|handle| {
            let packfile = &assets.get(handle).unwrap().0;
            let chunk = packfile.chunks().next().unwrap();
            unsafe { &*(chunk.bytes().as_ptr() as *const PackContent) }
        })
        .collect();

    let mut context = ContentContext::default();

    unsafe {
        for content in &content_all {
            assert_eq!(
                ContentFlags::from_bits_retain(content.flags.get()),
                ContentFlags::MANGLED
            );

            let data = content.content.as_ptr();

            for fixup in content.localOffsets.as_slice() {
                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let offset = usize::from_le(value.read_unaligned());
                value.write_unaligned(data as usize + offset);
            }

            for fixup in content.externalOffsets.as_slice() {
                let target = &content_all[fixup.targetFileIndex.get() as usize];

                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let offset = usize::from_le(value.read_unaligned());
                value.write_unaligned(target.content.as_ptr() as usize + offset);
            }

            for fixup in content.fileIndices.as_slice() {
                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let file_index = usize::from_le(value.read_unaligned());
                value.write_unaligned(
                    content_all[0].fileRefs.as_slice()[file_index].as_ptr() as usize
                );
            }

            for fixup in content.stringIndices.as_slice() {
                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let string_index = usize::from_le(value.read_unaligned());
                value.write_unaligned(content.strings.as_slice()[string_index].as_ptr() as usize);
            }

            for entry in content.indexEntries.as_slice() {
                let type_id = entry.r#type.get();
                let type_info = &content_all[0].typeInfos.as_slice()[type_id as usize];

                let data = &content.content.as_slice()[entry.offset.get() as usize..];

                context
                    .by_type
                    .entry(type_id)
                    .or_default()
                    .push(data.as_ptr());

                let guid_offset = type_info.guidOffset.get();
                if guid_offset != u32::MAX {
                    let guid = std::ptr::read(data[guid_offset as usize..].as_ptr().cast::<Guid>());

                    context.by_guid.insert(guid, data.as_ptr());
                }

                let data_id_offset = type_info.dataIdOffset.get();
                if data_id_offset != u32::MAX {
                    let data_id =
                        std::ptr::read(data[data_id_offset as usize..].as_ptr().cast::<u32>());

                    context
                        .by_data_id
                        .insert(type_id << 22 | data_id & 0x3FFFFF, data.as_ptr());
                }

                let name_offset = type_info.nameOffset.get();
                if name_offset != u32::MAX {
                    let name = std::ptr::read_unaligned(
                        data[name_offset as usize..].as_ptr().cast::<Ptr<Name>>(),
                    );

                    let mut value = 0u64;
                    for word in name.as_ref().unwrap().0.0.as_slice().iter() {
                        value = (value << 6)
                            | u64::from(match word.get() as u8 {
                                word @ b'A'..=b'Z' => word - b'A',
                                word @ b'a'..=b'z' => word - b'a' + 26,
                                word @ b'0'..=b'9' => word - b'0' + 52,
                                b'+' => 62,
                                b'/' => 63,
                                _ => continue,
                            });
                    }

                    context.by_name.insert(value, data.as_ptr());
                }
            }
        }
    }

    commands.insert_resource(context);
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ContentFlags: u32 {
        const ENCRYPTED = 1 << 0;
        const MANGLED   = 1 << 1;
    }
}

pub use ascalon_asset::packfile::{Guid, Token32, Token64, WcharPtr};

#[repr(transparent)]
pub struct Ptr<T: ?Sized>(*const T);

impl<T: ?Sized> Ptr<T> {
    pub fn as_ptr(&self) -> *const T {
        self.0
    }

    pub unsafe fn as_ref(&self) -> Option<&T> {
        unsafe { self.as_ptr().as_ref() }
    }
}

impl<T: ?Sized + std::fmt::Debug> std::fmt::Debug for Ptr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { self.as_ref() }.fmt(f)
    }
}

#[repr(C, align(8))]
pub struct String(WcharPtr, u32);

impl std::fmt::Debug for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[repr(C)]
pub struct Name(String, String);

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn mangle_name(name: &str) -> u64 {
    let mut digest = Sha256::new();
    for word in name.encode_utf16() {
        digest.update(word.to_le_bytes());
    }

    let mut hash = 0xCBF29CE484222325u64;
    for chunk in digest.finalize().chunks_exact(4) {
        let mut value = u32::from_le_bytes(chunk.try_into().unwrap());

        for _ in 0..4 {
            hash = (hash ^ value as u64).wrapping_mul(0x100000001B3);
            value >>= 8;
        }
    }

    hash.swap_bytes() >> 34
}

pub trait ContentType {
    const ID: u32;
}

mod achievement;
pub use achievement::*;

mod color;
pub use color::*;

mod configuration;
pub use configuration::*;

mod crafting_recipe;
pub use crafting_recipe::*;

mod currency;
pub use currency::*;

mod effect;
pub use effect::*;

mod emote;
pub use emote::*;

mod item;
pub use item::*;

mod mail;
pub use mail::*;

mod map;
pub use map::*;

mod marker;
pub use marker::*;

mod progress;
pub use progress::*;

mod sector;
pub use sector::*;

mod skill;
pub use skill::*;

mod skin;
pub use skin::*;

mod species;
pub use species::*;

mod table;
pub use table::*;

mod team;
pub use team::*;

mod r#trait;
pub use r#trait::*;

#[rustfmt::skip]
#[derive(Debug)]
#[repr(C, u32)]
pub enum Content {
    Achievement(Ptr<Achievement>)         = 0,
    _1                                    = 1,
    _2                                    = 2,
    _3                                    = 3,
    _4                                    = 4,
    _5                                    = 5,
    _6                                    = 6,
    _7                                    = 7,
    _8                                    = 8,
    Color(Ptr<Color>)                     = 9,
    _10                                   = 10,
    _11                                   = 11,
    CraftingRecipe(Ptr<CraftingRecipe>)   = 12,
    _13                                   = 13,
    Currency(Ptr<Currency>)               = 14,
    _15                                   = 15,
    _16                                   = 16,
    _17                                   = 17,
    _18                                   = 18,
    Emote(Ptr<Emote>)                     = 19,
    _20                                   = 20,
    _21                                   = 21,
    _22                                   = 22,
    _23                                   = 23,
    _24                                   = 24,
    _25                                   = 25,
    _26                                   = 26,
    GuildUpgrade                          = 27,
    _28                                   = 28,
    _29                                   = 29,
    _30                                   = 30,
    _31                                   = 31,
    _32                                   = 32,
    _33                                   = 33,
    _34                                   = 34,
    Item(Ptr<Item>)                       = 35,
    _36                                   = 36,
    _37                                   = 37,
    _38                                   = 38,
    _39                                   = 39,
    _40                                   = 40,
    _41                                   = 41,
    _42                                   = 42,
    Mail(Ptr<Mail>)                       = 43,
    _44                                   = 44,
    Map(Ptr<Map>)                         = 45,
    _46                                   = 46,
    _47                                   = 47,
    _48                                   = 48,
    _49                                   = 49,
    _50                                   = 50,
    _51                                   = 51,
    _52                                   = 52,
    Progress(Ptr<Progress>)               = 53,
    _54                                   = 54,
    _55                                   = 55,
    _56                                   = 56,
    _57                                   = 57,
    _58                                   = 58,
    _59                                   = 59,
    _60                                   = 60,
    _61                                   = 61,
    _62                                   = 62,
    Sector(Ptr<Sector>)                   = 63,
    Skill(Ptr<Skill>)                     = 64,
    _65                                   = 65,
    Skin(Ptr<Skin>)                       = 66,
    Species(Ptr<Species>)                 = 67,
    _68                                   = 68,
    _69                                   = 69,
    _70                                   = 70,
    _71                                   = 71,
    _72                                   = 72,
    _73                                   = 73,
    _74                                   = 74,
    _75                                   = 75,
    _76                                   = 76,
    Trait(Ptr<Trait>)                     = 77,
    _78                                   = 78,
    _79                                   = 79,
    _80                                   = 80,
    _81                                   = 81,
    _82                                   = 82,
    _83                                   = 83,
    _84                                   = 84,
    _85                                   = 85,
    _86                                   = 86,
    _87                                   = 87,
    _88                                   = 88,
    _89                                   = 89,
    _90                                   = 90,
    _91                                   = 91,
    _92                                   = 92,
    _93                                   = 93,
    _94                                   = 94,
    _95                                   = 95,
    _96                                   = 96,
    _97                                   = 97,
    _98                                   = 98,
    _99                                   = 99,
    AnimationListener                     = 100,
    AnimationBlendTree                    = 101,
    _102                                  = 102,
    _103                                  = 103,
    _104                                  = 104,
    _105                                  = 105,
    _106                                  = 106,
    _107                                  = 107,
    _108                                  = 108,
    _109                                  = 109,
    _110                                  = 110,
    _111                                  = 111,
    _112                                  = 112,
    _113                                  = 113,
    _114                                  = 114,
    _115                                  = 115,
    _116                                  = 116,
    _117                                  = 117,
    _118                                  = 118,
    _119                                  = 119,
    _120                                  = 120,
    _121                                  = 121,
    _122                                  = 122,
    _123                                  = 123,
    _124                                  = 124,
    _125                                  = 125,
    _126                                  = 126,
    _127                                  = 127,
    _128                                  = 128,
    _129                                  = 129,
    _130                                  = 130,
    _131                                  = 131,
    _132                                  = 132,
    _133                                  = 133,
    _134                                  = 134,
    _135                                  = 135,
    _136                                  = 136,
    _137                                  = 137,
    _138                                  = 138,
    _139                                  = 139,
    _140                                  = 140,
    _141                                  = 141,
    _142                                  = 142,
    _143                                  = 143,
    _144                                  = 144,
    _145                                  = 145,
    _146                                  = 146,
    ColorPalette(Ptr<ColorPalette>)       = 147,
    _148                                  = 148,
    _149                                  = 149,
    Configuration(Ptr<Configuration>)     = 150,
    _151                                  = 151,
    _152                                  = 152,
    _153                                  = 153,
    _154                                  = 154,
    _155                                  = 155,
    _156                                  = 156,
    _157                                  = 157,
    _158                                  = 158,
    _159                                  = 159,
    _160                                  = 160,
    _161                                  = 161,
    _162                                  = 162,
    _163                                  = 163,
    _164                                  = 164,
    _165                                  = 165,
    _166                                  = 166,
    _167                                  = 167,
    _168                                  = 168,
    _169                                  = 169,
    _170                                  = 170,
    _171                                  = 171,
    _172                                  = 172,
    _173                                  = 173,
    _174                                  = 174,
    _175                                  = 175,
    _176                                  = 176,
    _177                                  = 177,
    _178                                  = 178,
    _179                                  = 179,
    DynamicCamera                         = 180,
    DynamicCameraTransition               = 181,
    _182                                  = 182,
    Effect(Ptr<Effect>)                   = 183,
    _184                                  = 184,
    _185                                  = 185,
    _186                                  = 186,
    _187                                  = 187,
    _188                                  = 188,
    _189                                  = 189,
    _190                                  = 190,
    _191                                  = 191,
    _192                                  = 192,
    _193                                  = 193,
    _194                                  = 194,
    _195                                  = 195,
    _196                                  = 196,
    _197                                  = 197,
    _198                                  = 198,
    _199                                  = 199,
    _200                                  = 200,
    _201                                  = 201,
    _202                                  = 202,
    _203                                  = 203,
    _204                                  = 204,
    _205                                  = 205,
    _206                                  = 206,
    _207                                  = 207,
    _208                                  = 208,
    _209                                  = 209,
    _210                                  = 210,
    _211                                  = 211,
    _212                                  = 212,
    _213                                  = 213,
    _214                                  = 214,
    _215                                  = 215,
    _216                                  = 216,
    _217                                  = 217,
    _218                                  = 218,
    _219                                  = 219,
    _220                                  = 220,
    _221                                  = 221,
    _222                                  = 222,
    _223                                  = 223,
    _224                                  = 224,
    _225                                  = 225,
    _226                                  = 226,
    _227                                  = 227,
    _228                                  = 228,
    _229                                  = 229,
    _230                                  = 230,
    _231                                  = 231,
    _232                                  = 232,
    _233                                  = 233,
    _234                                  = 234,
    _235                                  = 235,
    _236                                  = 236,
    ItemConversionArray                   = 237,
    ItemCrest                             = 238,
    ItemDefault                           = 239,
    _240                                  = 240,
    _241                                  = 241,
    _242                                  = 242,
    _243                                  = 243,
    _244                                  = 244,
    _245                                  = 245,
    _246                                  = 246,
    _247                                  = 247,
    _248                                  = 248,
    _249                                  = 249,
    _250                                  = 250,
    _251                                  = 251,
    _252                                  = 252,
    _253                                  = 253,
    _254                                  = 254,
    _255                                  = 255,
    _256                                  = 256,
    _257                                  = 257,
    _258                                  = 258,
    _259                                  = 259,
    _260                                  = 260,
    _261                                  = 261,
    _262                                  = 262,
    _263                                  = 263,
    _264                                  = 264,
    _265                                  = 265,
    _266                                  = 266,
    _267                                  = 267,
    _268                                  = 268,
    _269                                  = 269,
    _270                                  = 270,
    _271                                  = 271,
    _272                                  = 272,
    _273                                  = 273,
    _274                                  = 274,
    _275                                  = 275,
    _276                                  = 276,
    _277                                  = 277,
    _278                                  = 278,
    _279                                  = 279,
    _280                                  = 280,
    _281                                  = 281,
    _282                                  = 282,
    _283                                  = 283,
    _284                                  = 284,
    _285                                  = 285,
    _286                                  = 286,
    _287                                  = 287,
    _288                                  = 288,
    _289                                  = 289,
    _290                                  = 290,
    _291                                  = 291,
    _292                                  = 292,
    Marker(Ptr<Marker>)                   = 293,
    _294                                  = 294,
    _295                                  = 295,
    _296                                  = 296,
    _297                                  = 297,
    _298                                  = 298,
    _299                                  = 299,
    _300                                  = 300,
    _301                                  = 301,
    _302                                  = 302,
    MovementModifier                      = 303,
    _304                                  = 304,
    MovementSettings                      = 305,
    _306                                  = 306,
    _307                                  = 307,
    _308                                  = 308,
    _309                                  = 309,
    _310                                  = 310,
    _311                                  = 311,
    _312                                  = 312,
    _313                                  = 313,
    _314                                  = 314,
    _315                                  = 315,
    _316                                  = 316,
    _317                                  = 317,
    _318                                  = 318,
    _319                                  = 319,
    _320                                  = 320,
    _321                                  = 321,
    _322                                  = 322,
    _323                                  = 323,
    _324                                  = 324,
    _325                                  = 325,
    _326                                  = 326,
    _327                                  = 327,
    _328                                  = 328,
    _329                                  = 329,
    _330                                  = 330,
    _331                                  = 331,
    _332                                  = 332,
    _333                                  = 333,
    _334                                  = 334,
    _335                                  = 335,
    _336                                  = 336,
    _337                                  = 337,
    _338                                  = 338,
    _339                                  = 339,
    _340                                  = 340,
    _341                                  = 341,
    _342                                  = 342,
    _343                                  = 343,
    _344                                  = 344,
    _345                                  = 345,
    _346                                  = 346,
    _347                                  = 347,
    _348                                  = 348,
    RandomUnlockTable                     = 349,
    _350                                  = 350,
    _351                                  = 351,
    _352                                  = 352,
    _353                                  = 353,
    _354                                  = 354,
    _355                                  = 355,
    _356                                  = 356,
    _357                                  = 357,
    _358                                  = 358,
    _359                                  = 359,
    _360                                  = 360,
    _361                                  = 361,
    _362                                  = 362,
    _363                                  = 363,
    _364                                  = 364,
    _365                                  = 365,
    _366                                  = 366,
    _367                                  = 367,
    _368                                  = 368,
    _369                                  = 369,
    _370                                  = 370,
    _371                                  = 371,
    _372                                  = 372,
    _373                                  = 373,
    _374                                  = 374,
    _375                                  = 375,
    _376                                  = 376,
    _377                                  = 377,
    _378                                  = 378,
    _379                                  = 379,
    _380                                  = 380,
    _381                                  = 381,
    _382                                  = 382,
    _383                                  = 383,
    _384                                  = 384,
    _385                                  = 385,
    _386                                  = 386,
    _387                                  = 387,
    _388                                  = 388,
    _389                                  = 389,
    _390                                  = 390,
    _391                                  = 391,
    _392                                  = 392,
    _393                                  = 393,
    Table(Ptr<TableInt>)                  = 394,
    _395                                  = 395,
    _396                                  = 396,
    _397                                  = 397,
    _398                                  = 398,
    _399                                  = 399,
    _400                                  = 400,
    Team(Ptr<Team>)                       = 401,
    TerrainEffectTable                    = 402,
    TerrainDecalTable                     = 403,
    _404                                  = 404,
    _405                                  = 405,
    _406                                  = 406,
    _407                                  = 407,
    _408                                  = 408,
    _409                                  = 409,
    _410                                  = 410,
    _411                                  = 411,
    _412                                  = 412,
    _413                                  = 413,
    _414                                  = 414,
    _415                                  = 415,
    _416                                  = 416,
    _417                                  = 417,
    _418                                  = 418,
    _419                                  = 419,
    _420                                  = 420,
    _421                                  = 421,
    _422                                  = 422,
    _423                                  = 423,
    _424                                  = 424,
    _425                                  = 425,
    _426                                  = 426,
    _427                                  = 427,
    _428                                  = 428,
    _429                                  = 429,
    _430                                  = 430,
    Boolean(bool)                         = 431,
    Enum(u32)                             = 432,
    Flags(u32)                            = 433,
    Integer(u32)                          = 434,
    IntegerPair(u32, u32)                 = 435,
    IntegerRange(u32, u32)                = 436,
    Number(f32)                           = 437,
    NumberPair(f32, f32)                  = 438,
    NumberRange(f32, f32)                 = 439,
    Point3d(f32, f32, f32)                = 440,
    String(WcharPtr)                      = 441,
    Text                                  = 442,
    TextCoded                             = 443,
    Time                                  = 444,
    TimeOfDay                             = 445,
    Token32(Token32)                      = 446,
    Token64(Token64)                      = 447,
}
