use std::collections::HashMap;
use std::fmt::Debug;

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
            init_content
                .run_if(content_handles_loaded)
                .run_if(not(resource_exists::<Content>)),
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

pub fn content_handles_loaded(
    asset_server: Res<AssetServer>,
    handles: Res<ContentHandles>,
) -> bool {
    asset_server.are_dependencies_loaded(&*handles)
}

#[derive(Resource)]
pub struct Content(HashMap<u32, *const u8>);

unsafe impl Sync for Content {}

unsafe impl Send for Content {}

impl Content {
    pub fn iter<'a, T: ContentType + 'a>(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.0.iter().filter_map(|(&key, &ptr)| {
            if key >> 22 != T::TYPE_ID {
                return None;
            }

            unsafe { (ptr as *const T).as_ref() }
        })
    }

    pub fn get<T: ContentType>(&self, data_id: u32) -> Option<&T> {
        unsafe { (*self.0.get(&(T::TYPE_ID << 22 | data_id))? as *const T).as_ref() }
    }
}

pub fn init_content(
    mut commands: Commands,
    assets: Res<Assets<Packfile>>,
    handles: Res<ContentHandles>,
) {
    let content: Vec<_> = handles
        .0
        .iter()
        .map(|handle| {
            let packfile = &assets.get(handle).unwrap().0;
            let chunk = packfile.chunks().next().unwrap();
            unsafe { &*(chunk.bytes().as_ptr() as *const PackContent) }
        })
        .collect();

    let mut index = HashMap::new();

    unsafe {
        for content_part in &content {
            let data = content_part.content.as_ptr();

            for fixup in content_part.localOffsets.as_slice() {
                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let offset = usize::from_le(value.read_unaligned());
                value.write_unaligned(data as usize + offset);
            }

            for fixup in content_part.externalOffsets.as_slice() {
                let target = &content[fixup.targetFileIndex.get() as usize];

                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let offset = usize::from_le(value.read_unaligned());
                value.write_unaligned(target.content.as_ptr() as usize + offset);
            }

            for fixup in content_part.fileIndices.as_slice() {
                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let file_index = usize::from_le(value.read_unaligned());
                value.write_unaligned(content[0].fileRefs.as_slice()[file_index].as_ptr() as usize);
            }

            for fixup in content_part.stringIndices.as_slice() {
                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let string_index = usize::from_le(value.read_unaligned());
                value.write_unaligned(
                    content_part.strings.as_slice()[string_index].as_ptr() as usize
                );
            }

            for entry in content_part.indexEntries.as_slice() {
                let type_id = entry.r#type.get();
                let data = &content_part.content.as_slice()[entry.offset.get() as usize..];

                let type_info = &content[0].typeInfos.as_slice()[type_id as usize];

                let data_id_offset = type_info.dataIdOffset.get();
                if data_id_offset != u32::MAX {
                    let data_id = u32::from_le_bytes(
                        data[data_id_offset as usize..][..4].try_into().unwrap(),
                    );
                    index.insert(type_id << 22 | data_id & 0x3FFFFF, data.as_ptr());
                }
            }
        }
    }

    commands.insert_resource(Content(index));
}

#[repr(transparent)]
pub struct WcharPtr(*const u16);

impl WcharPtr {
    pub fn as_ptr(&self) -> *const u16 {
        self.0
    }

    pub unsafe fn len(&self) -> usize {
        let mut ptr = self.as_ptr();
        if ptr.is_null() {
            return 0;
        }

        unsafe {
            while ptr.read_unaligned() != 0 {
                ptr = ptr.add(1);
            }

            ptr.offset_from_unsigned(self.as_ptr())
        }
    }

    pub unsafe fn as_slice(&self) -> &[u16] {
        let ptr = self.as_ptr();
        if ptr.is_null() {
            return &[];
        }

        unsafe { std::slice::from_raw_parts(ptr, self.len()) }
    }
}

impl WcharPtr {
    pub unsafe fn file_id(&self) -> Option<u32> {
        let [a, b, ..] = (unsafe { self.as_slice() }) else {
            return None;
        };

        if *a <= 0xFF || *b <= 0xFF {
            return None;
        }

        Some((u32::from(*a) - 0xFF) + (u32::from(*b) - 0x100) * 0xFF00)
    }
}

#[repr(C)]
pub struct String(WcharPtr, u32);

#[repr(C)]
pub struct Name {
    pub _00: String,
    pub _10: String,
}

pub trait ContentType {
    const TYPE_ID: u32;
}

mod achievement;
pub use achievement::*;

impl ContentType for Achievement {
    const TYPE_ID: u32 = 0x00;
}

mod color;
pub use color::*;

impl ContentType for Color {
    const TYPE_ID: u32 = 0x09;
}

mod crafting_recipe;
pub use crafting_recipe::*;

impl ContentType for CraftingRecipe {
    const TYPE_ID: u32 = 0x0C;
}

mod item;
pub use item::*;

impl ContentType for Item {
    const TYPE_ID: u32 = 0x23;
}

mod map;
pub use map::*;

impl ContentType for Map {
    const TYPE_ID: u32 = 0x2D;
}

mod progress;
pub use progress::*;

impl ContentType for Progress {
    const TYPE_ID: u32 = 0x35;
}

mod skill;
pub use skill::*;

impl ContentType for Skill {
    const TYPE_ID: u32 = 0x40;
}

mod skin;
pub use skin::*;

impl ContentType for Skin {
    const TYPE_ID: u32 = 0x42;
}
