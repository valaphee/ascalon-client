use ascalon_asset::packfile::Packfile;
use ascalon_asset::packfile::mapc::{
    MapParam, PackMapEnvironmentV78, PackMapLights, PackMapPropV21, PackMapTerrainV15,
};
use bevy::asset::io::{Reader, VecReader};
use bevy::asset::{AssetLoader, LoadContext, RenderAssetUsages};
use bevy::mesh::{Indices, PrimitiveTopology};
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

        load_environment(load_context, &packfile, &mut world);

        load_terrain(load_context, &packfile, &mut world);

        load_props(load_context, &packfile, &mut world);

        load_lights(load_context, &packfile, &mut world);

        return Ok(WorldAsset::new(world));
    }

    fn extensions(&self) -> &[&str] {
        &["amap2c"]
    }
}

fn load_environment(load_context: &mut LoadContext<'_>, packfile: &Packfile, world: &mut World) {
    let Some(environment) = packfile
        .chunks()
        .find(|chunk| chunk.name() == *b"env\0")
        .map(|chunk| unsafe { &*(chunk.bytes().as_ptr() as *const PackMapEnvironmentV78) })
    else {
        return;
    };
}

fn load_terrain(load_context: &mut LoadContext<'_>, packfile: &Packfile, world: &mut World) {
    let Some(param) = packfile
        .chunks()
        .find(|chunk| &chunk.name() == b"parm")
        .map(|chunk| unsafe { &*(chunk.bytes().as_ptr() as *const MapParam) })
    else {
        return;
    };

    let Some(terrain) = packfile
        .chunks()
        .find(|chunk| chunk.name() == *b"trn\0")
        .map(|chunk| unsafe { &*(chunk.bytes().as_ptr() as *const PackMapTerrainV15) })
    else {
        return;
    };

    let [min_x, min_y, max_x, max_y] = param.rect.map(|v| v.get() * unit::INCH_TO_M);
    let [dim_x, dim_y] = terrain.dims.map(|v| v.get() as f32 * unit::INCH_TO_M);

    let height_map_array = unsafe { terrain.heightMapArray.as_slice() };
    let chunk_array = unsafe { terrain.chunkArray.as_slice() };

    let chunk_count_x = (dim_x * chunk_array.len() as f32 / dim_y).sqrt() as usize;
    let chunk_count_y = chunk_array.len() / chunk_count_x;

    let chunk_size_x = (max_x - min_x) / chunk_count_x as f32;
    let chunk_size_y = (max_y - min_y) / chunk_count_y as f32;

    let vertices_per_chunk_side = terrain.verticesPerChunkSide.get() as usize;
    let vertex_count = vertices_per_chunk_side + 1;
    let sample_count = vertices_per_chunk_side + 3;

    let step_x = chunk_size_x / vertices_per_chunk_side as f32;
    let step_y = chunk_size_y / vertices_per_chunk_side as f32;

    let mut indices = Vec::with_capacity(
        2 * vertex_count * vertices_per_chunk_side + 2 * (vertices_per_chunk_side - 1),
    );

    for y in 0..vertices_per_chunk_side {
        let reversed = y % 2 != 0;

        for x in 0..vertex_count {
            let x = if reversed {
                vertices_per_chunk_side - x
            } else {
                x
            };

            let top = (y * vertex_count + x) as u16;
            let bottom = top + vertex_count as u16;

            if y > 0 && x == if reversed { vertices_per_chunk_side } else { 0 } {
                indices.extend_from_slice(&[
                    *indices.last().unwrap(),
                    if reversed { bottom } else { top },
                ]);
            }

            if reversed {
                indices.extend_from_slice(&[bottom, top]);
            } else {
                indices.extend_from_slice(&[top, bottom]);
            }
        }
    }

    let material = load_context.add_labeled_asset("terrain_material", StandardMaterial::default());

    for (index, samples) in height_map_array
        .chunks_exact(sample_count * sample_count)
        .take(chunk_array.len())
        .enumerate()
    {
        let height = |x: usize, y: usize| samples[y * sample_count + x].get() * unit::INCH_TO_M;

        let mut positions = Vec::with_capacity(vertex_count * vertex_count);
        let mut normals = Vec::with_capacity(vertex_count * vertex_count);

        for y in 0..vertex_count {
            for x in 0..vertex_count {
                let (sx, sy) = (x + 1, y + 1);

                positions.push([
                    x as f32 * step_x - chunk_size_x * 0.5,
                    -height(sx, sy),
                    y as f32 * step_y - chunk_size_y * 0.5,
                ]);

                let slope_x = (height(sx + 1, sy) - height(sx - 1, sy)) / (2.0 * step_x);
                let slope_y = (height(sx, sy + 1) - height(sx, sy - 1)) / (2.0 * step_y);

                normals.push(Vec3::new(slope_x, 1.0, slope_y).normalize().to_array());
            }
        }

        let mesh = Mesh::new(
            PrimitiveTopology::TriangleStrip,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U16(indices.clone()));

        let chunk_x = index % chunk_count_x;
        let chunk_y = chunk_count_y - 1 - index / chunk_count_x;

        let center_x = min_x + (chunk_x as f32 + 0.5) * chunk_size_x;
        let center_y = min_y + (chunk_y as f32 + 0.5) * chunk_size_y;

        world.spawn((
            Mesh3d(load_context.add_labeled_asset(format!("terrain_{index}_mesh"), mesh)),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(center_x, 0.0, -center_y),
        ));
    }
}

fn load_props(load_context: &mut LoadContext<'_>, packfile: &Packfile, world: &mut World) {
    let Some(prop) = packfile
        .chunks()
        .find(|chunk| &chunk.name() == b"prp2")
        .map(|chunk| unsafe { &*(chunk.bytes().as_ptr() as *const PackMapPropV21) })
    else {
        return;
    };

    for prop_obj in unsafe { prop.propArray.as_slice() } {
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

    for prop_obj in unsafe { prop.propAnimArray.as_slice() } {
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

    for prop_obj in unsafe { prop.propInstanceArray.as_slice() } {
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

fn load_lights(_load_context: &mut LoadContext<'_>, packfile: &Packfile, world: &mut World) {
    let Some(lights) = packfile
        .chunks()
        .find(|chunk| &chunk.name() == b"lght")
        .map(|chunk| unsafe { &*(chunk.bytes().as_ptr() as *const PackMapLights) })
    else {
        return;
    };

    for group in unsafe { lights.pointLights.as_slice() } {
        for light in unsafe { group.lights.as_slice() } {
            world.spawn((
                PointLight {
                    color: Color::srgb_u8(light.color[2], light.color[1], light.color[0]),
                    /*intensity: light.intensity.get(),*/
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

    for group in unsafe { lights.spotLights.as_slice() } {
        for light in unsafe { group.lights.as_slice() } {
            world.spawn((
                SpotLight {
                    color: Color::srgb_u8(light.color[2], light.color[1], light.color[0]),
                    /*intensity: light.intensity.get(),*/
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
