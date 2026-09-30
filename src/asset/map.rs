use ascalon_asset::packfile::Packfile;
use ascalon_asset::packfile::mapc::{PackMapPropV21, PackMapTerrainV15};
use bevy::asset::io::{Reader, VecReader};
use bevy::asset::{AssetLoader, LoadContext};
use bevy::reflect::TypePath;
use bevy::world_serialization::WorldAsset;

#[derive(Default, TypePath)]
pub struct MapLoader;

impl AssetLoader for MapLoader {
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
        if &packfile.r#type() != b"mapc" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid magic",
            ));
        }

        for chunk in packfile.chunks() {
            match &chunk.name() {
                b"trn\0" => {
                    let data = unsafe { &*(chunk.bytes().as_ptr() as *const PackMapTerrainV15) };
                }
                b"prp2" => {
                    let data = unsafe { &*(chunk.bytes().as_ptr() as *const PackMapPropV21) };
                }
                _ => {}
            }
        }

        return Err(std::io::ErrorKind::InvalidData.into());
    }
}
