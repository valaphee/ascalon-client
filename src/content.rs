use std::collections::HashMap;

use ascalon_asset::packfile::cntc::PackContent;
use bevy::app::{App, Plugin, Update};
use bevy::asset::{AssetServer, Assets, Handle, VisitAssetDependencies};
use bevy::ecs::resource::Resource;
use bevy::ecs::schedule::IntoScheduleConfigs as _;
use bevy::ecs::schedule::common_conditions::{not, resource_exists};
use bevy::ecs::system::{Commands, Res};
use bevy::ecs::world::{FromWorld, World};

use crate::asset::Packfile;

pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ContentHandles>().add_systems(
            Update,
            load_content
                .run_if(not(resource_exists::<Content>))
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
pub struct Content {
    by_type:    HashMap<u32, Vec<*const u8>>,
    by_guid:    HashMap<Guid, *const u8>,
    by_data_id: HashMap<u32, *const u8>,
}

unsafe impl Sync for Content {}
unsafe impl Send for Content {}

impl Content {
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

    let mut _content = Content::default();

    unsafe {
        for content in &content_all {
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

                _content
                    .by_type
                    .entry(type_id)
                    .or_default()
                    .push(data.as_ptr());

                let guid_offset = type_info.guidOffset.get();
                if guid_offset != u32::MAX {
                    let guid = std::ptr::read(data[guid_offset as usize..].as_ptr().cast::<Guid>());

                    _content.by_guid.insert(guid, data.as_ptr());
                }

                let data_id_offset = type_info.dataIdOffset.get();
                if data_id_offset != u32::MAX {
                    let data_id =
                        std::ptr::read(data[data_id_offset as usize..].as_ptr().cast::<u32>());

                    _content
                        .by_data_id
                        .insert(type_id << 22 | data_id & 0x3FFFFF, data.as_ptr());
                }
            }
        }
    }

    commands.insert_resource(_content);
}

pub use ascalon_asset::packfile::{Guid, WcharPtr};

#[repr(C, align(8))]
pub struct String(WcharPtr, u32);

#[repr(C)]
pub struct Name {
    pub _00: String,
    pub _10: String,
}

pub trait ContentType {
    const ID: u32;
}

mod achievement;
pub use achievement::*;

impl ContentType for Achievement {
    const ID: u32 = 0x00;
}

mod color;
pub use color::*;

impl ContentType for Color {
    const ID: u32 = 0x09;
}

mod crafting_recipe;
pub use crafting_recipe::*;

impl ContentType for CraftingRecipe {
    const ID: u32 = 0x0C;
}

mod currency;
pub use currency::*;

impl ContentType for Currency {
    const ID: u32 = 0x0E;
}

mod item;
pub use item::*;

impl ContentType for Item {
    const ID: u32 = 0x23;
}

mod map;
pub use map::*;

impl ContentType for Map {
    const ID: u32 = 0x2D;
}

mod progress;
pub use progress::*;

impl ContentType for Progress {
    const ID: u32 = 0x35;
}

mod skill;
pub use skill::*;

impl ContentType for Skill {
    const ID: u32 = 0x40;
}

mod skin;
pub use skin::*;

impl ContentType for Skin {
    const ID: u32 = 0x42;
}

mod configuration;
pub use configuration::*;

impl ContentType for Configuration {
    const ID: u32 = 0x96;
}
