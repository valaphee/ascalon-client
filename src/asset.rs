use std::path::Path;

use ascalon_asset::archive::Archive;
use bevy::app::{App, Plugin};
use bevy::asset::io::{
    AssetReaderError, AssetSourceBuilder, AssetSourceId, PathStream, Reader, VecReader,
};
use bevy::asset::{Asset, AssetApp as _, AssetLoader, LoadContext};
use bevy::reflect::TypePath;

use crate::asset::image::ImageLoader;
use crate::asset::map::MapLoader;
use crate::asset::model::ModelLoader;

mod image;
mod map;
mod model;

pub struct AssetSourcePlugin;

impl Plugin for AssetSourcePlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_source(
            AssetSourceId::Default,
            AssetSourceBuilder::new(|| {
                Box::new(AssetReader(
                    Archive::open("C:\\Program Files\\Guild Wars 2\\Gw2.dat").unwrap(),
                ))
            }),
        );
    }
}

struct AssetReader(Archive);

impl bevy::asset::io::AssetReader for AssetReader {
    async fn read<'a>(&'a self, path: &'a Path) -> Result<VecReader, AssetReaderError> {
        let id = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap()
            .parse::<u32>()
            .unwrap();

        Ok(VecReader::new(self.0.read(id)?))
    }

    async fn read_meta<'a>(
        &'a self,
        path: &'a std::path::Path,
    ) -> Result<VecReader, AssetReaderError> {
        Err(AssetReaderError::NotFound(path.to_path_buf()))
    }

    async fn read_directory<'a>(
        &'a self,
        path: &'a std::path::Path,
    ) -> Result<Box<PathStream>, AssetReaderError> {
        Err(AssetReaderError::NotFound(path.to_path_buf()))
    }

    async fn is_directory<'a>(
        &'a self,
        path: &'a std::path::Path,
    ) -> Result<bool, AssetReaderError> {
        Err(AssetReaderError::NotFound(path.to_path_buf()))
    }
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<ImageLoader>()
            .init_asset_loader::<MapLoader>()
            .init_asset_loader::<ModelLoader>()
            .init_asset::<Packfile>()
            .init_asset_loader::<PackfileLoader>();
    }
}

#[derive(Asset, TypePath)]
pub struct Packfile(pub ascalon_asset::packfile::Packfile);

#[derive(Default, TypePath)]
struct PackfileLoader;

impl AssetLoader for PackfileLoader {
    type Asset = Packfile;
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

        Ok(Packfile(ascalon_asset::packfile::Packfile::new(bytes)?))
    }
}
