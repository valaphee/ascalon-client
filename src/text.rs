use std::io::Read as _;
use std::sync::OnceLock;

use ascalon_asset::packfile::txtm::TextPackManifest;
use bevy::asset::io::{Reader, VecReader};
use bevy::asset::{AssetLoader, LoadContext, VisitAssetDependencies};
use bevy::prelude::*;

use crate::asset::Packfile;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Strings>()
            .init_asset_loader::<StringsLoader>()
            .init_resource::<TextPackManifestHandle>()
            .add_systems(
                Update,
                (
                    load_text_pack.run_if(not(resource_exists::<TextPack>)),
                    load_text.run_if(resource_exists::<TextPack>),
                ),
            );
    }
}

#[derive(Asset, TypePath)]
pub struct Strings(Vec<StringsEntry>);

enum StringsEntry {
    Encrypted {
        base:  u16,
        bits:  u16,
        bytes: Box<[u8]>,
    },
    String(String),
}

#[derive(Default, TypePath)]
struct StringsLoader;

impl AssetLoader for StringsLoader {
    type Asset = Strings;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let reader = unsafe { &mut *(reader as *mut dyn Reader as *mut VecReader) };
        let bytes = std::mem::take(&mut reader.bytes);

        let mut bytes = bytes.as_slice();
        let magic = bytes.read_array::<4>()?;
        if &magic != b"strs" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid magic",
            ));
        }

        let mut entries = Vec::new();
        while bytes.len() >= 6 {
            let size = bytes.read_le::<u16>()? as usize;
            if size - 2 > bytes.len() {
                return Err(std::io::ErrorKind::UnexpectedEof.into());
            }

            let base = bytes.read_le::<u16>()?;
            let bits = bytes.read_le::<u16>()?;

            let _bytes = bytes.split_at(size - 6);
            bytes = _bytes.1;

            entries.push(if base == 0 {
                StringsEntry::String(
                    String::from_utf16le(_bytes.0).map_err(|_| std::io::ErrorKind::InvalidData)?,
                )
            } else {
                StringsEntry::Encrypted {
                    base,
                    bits,
                    bytes: _bytes.0.to_vec().into_boxed_slice(),
                }
            });
        }

        Ok(Strings(entries))
    }
}

#[derive(Resource, VisitAssetDependencies)]
struct TextPackManifestHandle(#[dependency] Handle<Packfile>);

impl FromWorld for TextPackManifestHandle {
    fn from_world(world: &mut World) -> Self {
        Self(world.resource::<AssetServer>().load("110865"))
    }
}

#[derive(Resource)]
struct TextPack {
    manifest: *const TextPackManifest,
    language: usize,
    strings:  Box<[OnceLock<Handle<Strings>>]>,
}

unsafe impl Send for TextPack {}
unsafe impl Sync for TextPack {}

impl TextPack {
    fn new(manifest: &TextPackManifest) -> Self {
        let language = unsafe { &manifest.languages.as_slice()[0] };
        let filenames = unsafe { language.filenames.as_slice() };

        Self {
            manifest: manifest as *const _,
            language: 0,
            strings:  vec![OnceLock::new(); filenames.len()].into_boxed_slice(),
        }
    }
}

fn load_text_pack(
    mut commands: Commands,
    assets: Res<Assets<Packfile>>,
    handle: Res<TextPackManifestHandle>,
) {
    let Some(asset) = assets.get(&handle.0) else {
        return;
    };
    assert_eq!(asset.r#type(), *b"txtm");

    let Some(manifest) = asset
        .chunks()
        .find(|chunk| chunk.name() == *b"txtm")
        .map(|chunk| unsafe { &*(chunk.bytes().as_ptr() as *const TextPackManifest) })
    else {
        panic!()
    };

    commands.insert_resource(TextPack::new(manifest));
}

#[derive(Component)]
pub struct Text(Vec<u16>);

#[derive(Component)]
struct TextPending;

impl Text {
    pub fn new(text_id: u32) -> Self {
        let mut buf = Vec::new();

        encode_numeric(&mut buf, text_id as u64);

        Self(buf)
    }
}

fn load_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Strings>>,
    text_pack: Res<TextPack>,
    mut texts: Query<
        (Entity, &Text, Option<&mut bevy::ui::widget::Text>),
        Or<(Changed<Text>, With<TextPending>)>,
    >,
) {
    let manifest = unsafe { &*text_pack.manifest };
    let strings_per_file = manifest.stringsPerFile.get();

    let Some(language) = (unsafe { manifest.languages.as_slice().get(text_pack.language) }) else {
        return;
    };

    let filenames = unsafe { language.filenames.as_slice() };

    for (entity, text, widget) in &mut texts {
        let mut words = text.0.as_slice();

        let Ok(text_id) = decode_numeric(&mut words).map(|v| v as u32) else {
            commands.entity(entity).remove::<TextPending>();
            continue;
        };

        let file_index = (text_id / strings_per_file) as usize;
        let string_index = (text_id % strings_per_file) as usize;

        let Some((handle_slot, filename)) = text_pack
            .strings
            .get(file_index)
            .zip(filenames.get(file_index))
        else {
            commands.entity(entity).remove::<TextPending>();
            continue;
        };

        let handle = handle_slot
            .get_or_init(|| asset_server.load(unsafe { filename.file_id() }.unwrap().to_string()));

        let Some(strings) = assets.get(handle) else {
            commands.entity(entity).insert(TextPending);
            continue;
        };

        commands.entity(entity).remove::<TextPending>();

        let Some(StringsEntry::String(value)) = strings.0.get(string_index) else {
            continue;
        };

        if let Some(mut text) = widget {
            text.0.clone_from(value);
        }
    }
}

const WORD_VALUE_BASE: u16 = 0x0100;
const WORD_VALUE_RANGE: u64 = 0x7F00;
const WORD_BIT_MORE: u16 = 0x8000;

fn encode_numeric(words: &mut Vec<u16>, value: u64) {
    let mut divisor = 1;
    while value / divisor >= WORD_VALUE_RANGE {
        divisor *= WORD_VALUE_RANGE;
    }

    while divisor != 1 {
        let word = (value / divisor) % WORD_VALUE_RANGE;
        words.push((word as u16 + WORD_VALUE_BASE) | WORD_BIT_MORE);
        divisor /= WORD_VALUE_RANGE;
    }

    words.push((value % WORD_VALUE_RANGE) as u16 + WORD_VALUE_BASE);
}

fn decode_numeric(words: &mut &[u16]) -> std::io::Result<u64> {
    let mut value = 0u64;

    loop {
        let (word, _words) = words
            .split_first()
            .ok_or(std::io::ErrorKind::UnexpectedEof)?;
        *words = _words;

        let part = word & !WORD_BIT_MORE;
        if part < WORD_VALUE_BASE {
            return Err(std::io::ErrorKind::InvalidData.into());
        }

        value += (part - WORD_VALUE_BASE) as u64;

        if word & WORD_BIT_MORE == 0 {
            return Ok(value);
        }

        value *= WORD_VALUE_RANGE;
    }
}
