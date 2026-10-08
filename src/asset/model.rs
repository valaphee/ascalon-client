use ascalon_asset::packfile::Packfile;
use bevy::asset::io::{Reader, VecReader};
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;

#[derive(Default, TypePath)]
pub struct ModelLoader;

impl AssetLoader for ModelLoader {
    type Asset = WorldAsset;
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

        let packfile = Packfile::new(bytes)?;
        if &packfile.r#type() != b"MODL" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid magic",
            ));
        }

        todo!()
    }

    fn extensions(&self) -> &[&str] {
        &["amdl2"]
    }
}
