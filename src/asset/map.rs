use ascalon_asset::packfile::Packfile;
use ascalon_asset::packfile::mapc::{PackMapLights, PackMapPropV21};
use bevy::asset::io::{Reader, VecReader};
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;

use crate::unit;

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

        if let Some(chunk) = packfile
            .chunks()
            .find(|chunk| &chunk.name() == b"prp2")
            .map(|chunk| unsafe { &*(chunk.bytes().as_ptr() as *const PackMapPropV21) })
        {
            for prop_obj in unsafe { chunk.propArray.as_slice() } {
                let model = load_context.load(format!(
                    "{}.amdl2",
                    unsafe { prop_obj.filename.file_id() }.unwrap()
                ));

                world.spawn((
                    WorldAssetRoot(model),
                    Transform::default()
                        .with_translation(
                            unit::position([
                                prop_obj.position[0].get(),
                                prop_obj.position[1].get(),
                                prop_obj.position[2].get(),
                            ])
                            .into(),
                        )
                        .with_rotation(unit::rotation([
                            prop_obj.rotation[0].get(),
                            prop_obj.rotation[1].get(),
                            prop_obj.rotation[2].get(),
                        ]))
                        .with_scale(Vec3::splat(prop_obj.scale.get())),
                ));
            }

            for prop_obj in unsafe { chunk.propAnimArray.as_slice() } {
                let model = load_context.load(format!(
                    "{}.amdl2",
                    unsafe { prop_obj.filename.file_id() }.unwrap()
                ));

                world.spawn((
                    WorldAssetRoot(model),
                    Transform::default()
                        .with_translation(
                            unit::position([
                                prop_obj.position[0].get(),
                                prop_obj.position[1].get(),
                                prop_obj.position[2].get(),
                            ])
                            .into(),
                        )
                        .with_rotation(unit::rotation([
                            prop_obj.rotation[0].get(),
                            prop_obj.rotation[1].get(),
                            prop_obj.rotation[2].get(),
                        ]))
                        .with_scale(Vec3::splat(prop_obj.scale.get())),
                ));
            }

            for prop_obj in unsafe { chunk.propInstanceArray.as_slice() } {
                let model = load_context.load(format!(
                    "{}.amdl2",
                    unsafe { prop_obj.filename.file_id() }.unwrap()
                ));

                world.spawn((
                    WorldAssetRoot(model.clone()),
                    Transform::default()
                        .with_translation(
                            unit::position([
                                prop_obj.position[0].get(),
                                prop_obj.position[1].get(),
                                prop_obj.position[2].get(),
                            ])
                            .into(),
                        )
                        .with_rotation(unit::rotation([
                            prop_obj.rotation[0].get(),
                            prop_obj.rotation[1].get(),
                            prop_obj.rotation[2].get(),
                        ]))
                        .with_scale(Vec3::splat(prop_obj.scale.get())),
                ));

                for transform in unsafe { prop_obj.transforms.as_slice() } {
                    world.spawn((
                        WorldAssetRoot(model.clone()),
                        Transform::default()
                            .with_translation(
                                unit::position([
                                    transform.position[0].get(),
                                    transform.position[1].get(),
                                    transform.position[2].get(),
                                ])
                                .into(),
                            )
                            .with_rotation(unit::rotation([
                                transform.rotation[0].get(),
                                transform.rotation[1].get(),
                                transform.rotation[2].get(),
                            ]))
                            .with_scale(Vec3::splat(transform.scale.get())),
                    ));
                }
            }
        }

        if let Some(chunk) = packfile
            .chunks()
            .find(|chunk| &chunk.name() == b"lght")
            .map(|chunk| unsafe { &*(chunk.bytes().as_ptr() as *const PackMapLights) })
        {
            for group in unsafe { chunk.pointLights.as_slice() } {
                for light in unsafe { group.lights.as_slice() } {
                    world.spawn((
                        PointLight {
                            color: Color::srgb_u8(light.color[2], light.color[1], light.color[0]),
                            range: light.farDistance.get() * unit::INCH_TO_M,
                            ..default()
                        },
                        Transform::from_translation(Vec3::from(unit::position([
                            light.position[0].get(),
                            light.position[1].get(),
                            light.position[2].get(),
                        ]))),
                    ));
                }
            }

            for group in unsafe { chunk.spotLights.as_slice() } {
                for light in unsafe { group.lights.as_slice() } {
                    world.spawn((
                        SpotLight {
                            color: Color::srgb_u8(light.color[2], light.color[1], light.color[0]),
                            range: light.farDistance.get() * unit::INCH_TO_M,
                            outer_angle: light.outerAngle.get(),
                            inner_angle: light.innerAngle.get(),
                            ..default()
                        },
                        Transform::from_translation(Vec3::from(unit::position([
                            light.position[0].get(),
                            light.position[1].get(),
                            light.position[2].get(),
                        ])))
                        .looking_to(
                            Vec3::from(unit::direction([
                                light.direction[0].get(),
                                light.direction[1].get(),
                                light.direction[2].get(),
                            ])),
                            Vec3::from(unit::direction([
                                light.upDirection[0].get(),
                                light.upDirection[1].get(),
                                light.upDirection[2].get(),
                            ])),
                        ),
                    ));
                }
            }
        }

        return Ok(WorldAsset::new(world));
    }

    fn extensions(&self) -> &[&str] {
        &["amap2c"]
    }
}
