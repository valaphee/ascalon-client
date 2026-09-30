use ascalon_asset::packfile::Packfile;
use ascalon_asset::packfile::modl::{ModelFileDataV70, ModelFileGeometryV1};
use bevy::asset::io::{Reader, VecReader};
use bevy::asset::{AssetLoader, LoadContext};
use bevy::mesh::Mesh;
use bevy::reflect::TypePath;

#[derive(Default, TypePath)]
pub struct ModelLoader;

impl AssetLoader for ModelLoader {
    type Asset = Mesh;
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

        for chunk in packfile.chunks() {
            match &chunk.name() {
                b"MODL" => {
                    let data = unsafe { &*(chunk.bytes().as_ptr() as *const ModelFileDataV70) };
                }
                b"GEOM" => {
                    let data = unsafe { &*(chunk.bytes().as_ptr() as *const ModelFileGeometryV1) };
                }
                _ => {}
            }
        }

        return Err(std::io::ErrorKind::InvalidData.into());
    }
}
