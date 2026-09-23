use std::collections::HashMap;

use ascalon_asset::packfile::cntc::PackContent;
use bevy::{asset::VisitAssetDependencies, prelude::*};
use zerocopy::FromBytes;

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
    let all_content: Vec<_> = handles
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
    for content in &all_content {
        let data = content.content.as_ptr();

        for fixup in content.localOffsets.as_slice() {
            unsafe {
                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let offset = usize::from_le(value.read_unaligned());
                value.write_unaligned(data as usize + offset);
            }
        }

        for fixup in content.externalOffsets.as_slice() {
            let target = &all_content[fixup.targetFileIndex.get() as usize];

            unsafe {
                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let offset = usize::from_le(value.read_unaligned());
                value.write_unaligned(target.content.as_ptr() as usize + offset);
            }
        }

        for fixup in content.fileIndices.as_slice() {
            unsafe {
                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let file_index = usize::from_le(value.read_unaligned());
                value.write_unaligned(
                    all_content[0].fileRefs.as_slice()[file_index].as_ptr() as usize
                );
            }
        }

        for fixup in content.stringIndices.as_slice() {
            unsafe {
                let value = data
                    .add(fixup.relocOffset.get() as usize)
                    .cast_mut()
                    .cast::<usize>();
                let string_index = usize::from_le(value.read_unaligned());
                value.write_unaligned(content.strings.as_slice()[string_index].as_ptr() as usize);
            }
        }

        for entry in content.indexEntries.as_slice() {
            if entry.r#type != 0x23 {
                continue;
            }

            let type_info = &all_content[0].typeInfos.as_slice()[entry.r#type.get() as usize];

            let data = &content.content.as_slice()[entry.offset.get() as usize..];

            let data_id_offset = type_info.dataIdOffset.get();
            if data_id_offset != u32::MAX {
                let data_id =
                    u32::from_le_bytes(data[data_id_offset as usize..][..4].try_into().unwrap());
                index.insert(data_id, data.as_ptr());
            }
        }
    }

    commands.insert_resource(Content(index));
}
