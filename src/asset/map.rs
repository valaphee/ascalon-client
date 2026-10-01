use ascalon_asset::packfile::Packfile;
use ascalon_asset::packfile::mapc::PackMapPropV21;
use bevy::asset::io::{Reader, VecReader};
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;

use crate::coord;

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
        load_context: &mut LoadContext<'_>,
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

        let mut world = World::new();

        let material = load_context.add_labeled_asset(
            "material",
            StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 1.0,
                ..default()
            },
        );

        if let Some(prop) = packfile
            .chunks()
            .find(|chunk| &chunk.name() == b"prp2")
            .map(|chunk| unsafe { &*(chunk.bytes().as_ptr() as *const PackMapPropV21) })
        {
            for prop_obj in unsafe { prop.propArray.as_slice() } {
                let mesh =
                    load_context.load(unsafe { prop_obj.filename.file_id() }.unwrap().to_string());

                world.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(material.clone()),
                    Transform::default()
                        .with_translation(
                            coord::position([
                                prop_obj.position[0].get(),
                                prop_obj.position[1].get(),
                                prop_obj.position[2].get(),
                            ])
                            .into(),
                        )
                        .with_rotation(coord::rotation([
                            prop_obj.rotation[0].get(),
                            prop_obj.rotation[1].get(),
                            prop_obj.rotation[2].get(),
                        ]))
                        .with_scale(Vec3::splat(prop_obj.scale.get())),
                ));
            }

            for prop_obj in unsafe { prop.propInstanceArray.as_slice() } {
                let mesh =
                    load_context.load(unsafe { prop_obj.filename.file_id() }.unwrap().to_string());

                for transform in unsafe { prop_obj.transforms.as_slice() } {
                    world.spawn((
                        Mesh3d(mesh.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::default()
                            .with_translation(
                                coord::position([
                                    transform.position[0].get(),
                                    transform.position[1].get(),
                                    transform.position[2].get(),
                                ])
                                .into(),
                            )
                            .with_rotation(coord::rotation([
                                transform.rotation[0].get(),
                                transform.rotation[1].get(),
                                transform.rotation[2].get(),
                            ]))
                            .with_scale(Vec3::splat(transform.scale.get())),
                    ));
                }
            }
        }

        return Ok(WorldAsset::new(world));
    }
}
