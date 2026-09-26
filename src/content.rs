use std::{collections::HashMap, fmt};

use ascalon_asset::packfile::{WcharPtr, cntc::PackContent};
use bevy::{asset::VisitAssetDependencies, prelude::*};
use zerocopy::FromBytes;

use crate::asset::Packfile;

mod item;
mod map;

pub use {item::*, map::*};

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
                "꜍ē", "꜎ē", "꜏ē", "꜐ē", "꜑ē", "꜒ē", "꜓ē", "꜔ē", "꜕ē", "꜖ē", "ꜗē", "ꜘē", "ꜙē", "ꜚē",
                "ꜛē", "ꜜē", "ꜝē", "ꜞē", "ꜟē", "꜠ē", "꜡ē", "Ꜣē", "ꜣē", "Ꜥē", "ꜥē", "Ꜧē", "ꜧē", "Ꜩē",
                "ꜩē", "Ꜫē", "ꜫē", "Ꜭē",
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
                    index.insert(type_id << 22 | data_id & 0x3FFFFF, data.as_ptr());
                }
            }
        }
    }

    commands.insert_resource(Content(index));
}

pub trait ContentType {
    const TYPE_ID: u32;
}

impl ContentType for Item {
    const TYPE_ID: u32 = 0x23;
}

impl ContentType for Map {
    const TYPE_ID: u32 = 0x2D;
}

#[repr(C)]
pub struct Guid(u32, u16, u16, [u8; 8]);

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.0,
            self.1,
            self.2,
            self.3[0],
            self.3[1],
            self.3[2],
            self.3[3],
            self.3[4],
            self.3[5],
            self.3[6],
            self.3[7],
        )
    }
}

impl fmt::Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Name {
    pub _00: WcharPtr,
    pub _08: u32,
    pub _0c: u32,
    pub _10: WcharPtr,
    pub _18: u32,
    pub _1c: u32,
}
