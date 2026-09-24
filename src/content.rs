use std::collections::HashMap;

use ascalon_asset::packfile::{Ptr, WcharPtr, cntc::PackContent};
use bevy::{asset::VisitAssetDependencies, prelude::*};
use zerocopy::{
    FromBytes,
    little_endian::{U32, U64},
};

use crate::asset::Packfile;

pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ContentHandles>().add_systems(
            Update,
            load_content
                .run_if(content_sources_loaded)
                .run_if(not(resource_exists::<Content>)),
        );
    }
}

#[derive(Resource, VisitAssetDependencies)]
pub struct ContentHandles(#[dependency] Vec<Handle<Packfile>>);

impl FromWorld for ContentHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let handles = [
            "꜍ē", "꜎ē", "꜏ē", "꜐ē", "꜑ē", "꜒ē", "꜓ē", "꜔ē", "꜕ē", "꜖ē", "ꜗē", "ꜘē", "ꜙē", "ꜚē",
            "ꜛē", "ꜜē", "ꜝē", "ꜞē", "ꜟē", "꜠ē", "꜡ē", "Ꜣē", "ꜣē", "Ꜥē", "ꜥē", "Ꜧē", "ꜧē", "Ꜩē",
            "ꜩē", "Ꜫē", "ꜫē", "Ꜭē",
        ]
        .into_iter()
        .map(|path| asset_server.load(path))
        .collect();

        Self(handles)
    }
}

pub fn content_sources_loaded(
    asset_server: Res<AssetServer>,
    sources: Res<ContentHandles>,
) -> bool {
    asset_server.are_dependencies_loaded(&*sources)
}

#[derive(Resource)]
pub struct Content(HashMap<u32, *const u8>);

unsafe impl Sync for Content {}

unsafe impl Send for Content {}

pub fn load_content(
    assets: Res<Assets<Packfile>>,
    handles: Res<ContentHandles>,
    mut commands: Commands,
) {
    let content: Vec<_> = handles
        .0
        .iter()
        .map(|handle| {
            PackContent::ref_from_prefix(
                assets
                    .get(handle)
                    .unwrap()
                    .0
                    .chunks()
                    .nth(0)
                    .unwrap()
                    .bytes(),
            )
            .unwrap()
            .0
        })
        .collect();

    let mut index = HashMap::new();

    unsafe {
        for content_chunk in &content {
            let data = content_chunk.content.as_ptr();

            for fixup in content_chunk.localOffsets.as_slice() {
                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let offset = usize::from_le(value.read_unaligned());
                value.write_unaligned(data as usize + offset);
            }

            for fixup in content_chunk.externalOffsets.as_slice() {
                let target = &content[fixup.targetFileIndex.get() as usize];

                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let offset = usize::from_le(value.read_unaligned());
                value.write_unaligned(target.content.as_ptr() as usize + offset);
            }

            for fixup in content_chunk.fileIndices.as_slice() {
                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let file_index = usize::from_le(value.read_unaligned());
                value.write_unaligned(content[0].fileRefs.as_slice()[file_index].as_ptr() as usize);
            }

            for fixup in content_chunk.stringIndices.as_slice() {
                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let string_index = usize::from_le(value.read_unaligned());
                value.write_unaligned(
                    content_chunk.strings.as_slice()[string_index].as_ptr() as usize
                );
            }

            for entry in content_chunk.indexEntries.as_slice() {
                let type_id = entry.r#type.get();
                let data = &content_chunk.content.as_slice()[entry.offset.get() as usize..];

                let type_info = &content[0].typeInfos.as_slice()[type_id as usize];

                let data_id_offset = type_info.dataIdOffset.get();
                if data_id_offset != u32::MAX {
                    let data_id = u32::from_le_bytes(
                        data[data_id_offset as usize..][..4].try_into().unwrap(),
                    );
                    index.insert(type_id << 22 | data_id & 0x3fffff, data.as_ptr());
                }
            }
        }
    }

    commands.insert_resource(Content(index));
}

#[derive(Debug)]
#[repr(C)]
struct Name {
    _0: WcharPtr,
    _1: U64,
    _2: WcharPtr,
    _3: U64,
}

#[derive(Debug)]
#[repr(C)]
struct Item {
    _0: [u8; 16],
    _1: U64,
    _2: Ptr<Name>,
    _3: Ptr<Name>,
    dataId: U32,
    r#type: ItemType,
    flags: U64,
    icon: WcharPtr,
    _8: U32,
    _9: U32,
    _10: U32,
    _11: U32,
    _12: U32,
    _13: U32,
    rarity: U32,
    _15: U32,
    _16: U32,
    _17: U32,
    _18: U32,
    _19: U32,
    _20: U32,
    _21: U32,
    name: U32,
    description: U32,
}

#[derive(Debug)]
#[repr(C, u32)]
enum ItemType {
    Armor(Ptr<ItemArmor>),
    Augment(Ptr<ItemAugment>),
    Back(Ptr<ItemBack>),
    Bag(Ptr<ItemBag>),
    Consumable(Ptr<ItemConsumable>),
    Container(Ptr<ItemContainer>),
    CraftingMaterial(Ptr<ItemCraftingMaterial>),
    _7(U64),
    _8(U64),
    Gathering(Ptr<ItemGathering>),
    Gizmo(Ptr<ItemGizmo>),
    JadeTechModule(Ptr<ItemJadeTechModule>),
    _12(U64),
    _13(U64),
    _14(U64),
    MiniPet(Ptr<ItemMiniPet>),
    _16(U64),
    PowerCore(Ptr<ItemPowerCore>),
    Relic(Ptr<ItemRelic>),
    Tool(Ptr<ItemTool>),
    TraitGuide(Ptr<ItemTraitGuide>),
    Trinket(Ptr<ItemTrinket>),
    Trophy(Ptr<ItemTrophy>),
    UpgradeComponent(Ptr<ItemUpgradeComponent>),
    Weapon(Ptr<ItemWeapon>),
}

#[repr(C)]
#[derive(Debug)]
struct ItemArmor;

#[repr(C)]
#[derive(Debug)]
struct ItemAugment;

#[repr(C)]
#[derive(Debug)]
struct ItemBack;

#[repr(C)]
#[derive(Debug)]
struct ItemBag;

#[repr(C)]
#[derive(Debug)]
struct ItemConsumable;

#[repr(C)]
#[derive(Debug)]
struct ItemContainer;

#[repr(C)]
#[derive(Debug)]
struct ItemCraftingMaterial;

#[repr(C)]
#[derive(Debug)]
struct ItemGathering;

#[repr(C)]
#[derive(Debug)]
struct ItemGizmo;

#[repr(C)]
#[derive(Debug)]
struct ItemJadeTechModule;

#[repr(C)]
#[derive(Debug)]
struct ItemMiniPet;

#[repr(C)]
#[derive(Debug)]
struct ItemPowerCore;

#[repr(C)]
#[derive(Debug)]
struct ItemRelic;

#[repr(C)]
#[derive(Debug)]
struct ItemTool;

#[repr(C)]
#[derive(Debug)]
struct ItemTraitGuide;

#[repr(C)]
#[derive(Debug)]
struct ItemTrinket;

#[repr(C)]
#[derive(Debug)]
struct ItemTrophy;

#[repr(C)]
#[derive(Debug)]
struct ItemUpgradeComponent;

#[repr(C)]
#[derive(Debug)]
struct ItemWeapon;
