use std::io::Read as _;
use std::sync::OnceLock;

use ascalon_asset::packfile::txtm::TextPackManifest;
use bevy::asset::io::{Reader, VecReader};
use bevy::asset::{AssetLoader, LoadContext, VisitAssetDependencies};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use zerocopy::FromBytes;

use crate::asset::Packfile;

pub struct StringsPlugin;

impl Plugin for StringsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<StringsChunk>()
            .init_asset_loader::<StringsChunkLoader>()
            .init_resource::<StringsHandle>()
            .add_systems(
                Update,
                init_strings.run_if(not(resource_exists::<StringsState>)),
            );
    }
}

#[derive(Resource, VisitAssetDependencies)]
struct StringsHandle(#[dependency] Handle<Packfile>);

impl FromWorld for StringsHandle {
    fn from_world(world: &mut World) -> Self {
        Self(world.resource::<AssetServer>().load("댐ā"))
    }
}

enum StringsChunkEntry {
    Encrypted {
        offset: u16,
        bits: u16,
        data: Box<[u8]>,
    },
    String(String),
}

#[derive(Asset, TypePath)]
pub struct StringsChunk(Vec<StringsChunkEntry>);

#[derive(Default, TypePath)]
struct StringsChunkLoader;

impl AssetLoader for StringsChunkLoader {
    type Asset = StringsChunk;
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
                "strings: invalid magic",
            ));
        }

        let mut entries = Vec::new();
        while bytes.len() >= 6 {
            let size = bytes.read_le::<u16>()? as usize;
            if size - 2 > bytes.len() {
                return Err(std::io::ErrorKind::UnexpectedEof.into());
            }

            let offset = bytes.read_le::<u16>()?;
            let bits = bytes.read_le::<u16>()?;

            let _bytes = bytes.split_at(size - 6);
            bytes = _bytes.1;

            entries.push(if offset == 0 {
                StringsChunkEntry::String(
                    String::from_utf16le(_bytes.0).map_err(|_| std::io::ErrorKind::InvalidData)?,
                )
            } else {
                StringsChunkEntry::Encrypted {
                    offset,
                    bits,
                    data: _bytes.0.to_vec().into_boxed_slice(),
                }
            });
        }

        Ok(StringsChunk(entries))
    }
}

#[derive(SystemParam)]
pub struct Strings<'w> {
    state: Res<'w, StringsState>,
    asset_server: Res<'w, AssetServer>,
    assets: Res<'w, Assets<StringsChunk>>,
}

impl Strings<'_> {
    pub fn get(&self, text_id: u32) -> Option<&str> {
        let manifest = unsafe { &*self.state.manifest };

        let strings_per_file = manifest.stringsPerFile.get();
        let file_index = (text_id / strings_per_file) as usize;
        let string_index = (text_id % strings_per_file) as usize;

        let language = unsafe { manifest.languages.as_slice().get(self.state.language)? };
        let filenames = unsafe { language.filenames.as_slice() };

        let handle = self.state.strings[file_index].get_or_init(|| {
            self.asset_server
                .load(unsafe { filenames[file_index].to_string_lossy() })
        });

        self.assets
            .get(&*handle)?
            .0
            .get(string_index)
            .and_then(|entry| match entry {
                StringsChunkEntry::String(value) => Some(value.as_str()),
                StringsChunkEntry::Encrypted { .. } => None,
            })
    }
}

#[derive(Resource)]
struct StringsState {
    manifest: *const TextPackManifest,
    language: usize,
    strings: Box<[OnceLock<Handle<StringsChunk>>]>,
}

unsafe impl Sync for StringsState {}

unsafe impl Send for StringsState {}

fn init_strings(mut commands: Commands, assets: Res<Assets<Packfile>>, handle: Res<StringsHandle>) {
    let Some(asset) = assets.get(&handle.0) else {
        return;
    };

    let manifest = TextPackManifest::ref_from_prefix(asset.0.chunks().nth(0).unwrap().bytes())
        .unwrap()
        .0;
    let language = unsafe { &manifest.languages.as_slice()[0] };
    let filenames = unsafe { language.filenames.as_slice() };

    commands.insert_resource(StringsState {
        manifest: manifest as *const _,
        language: 0,
        strings: vec![OnceLock::new(); filenames.len()].into_boxed_slice(),
    });
}
