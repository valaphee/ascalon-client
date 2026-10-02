use std::io::Read as _;

use ascalon_asset::packfile::Packfile;
use ascalon_asset::packfile::modl::{ModelFileGeometryV1, ModelMeshDataV66};
use bevy::asset::io::{Reader, VecReader};
use bevy::asset::{AssetLoader, LoadContext, RenderAssetUsages};
use bevy::mesh::{Indices, VertexAttributeValues};
use bevy::prelude::*;
use bitflags::bitflags;

use crate::coord;

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
        load_context: &mut LoadContext<'_>,
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

        let Some(file_geometry) = packfile
            .chunks()
            .find(|chunk| &chunk.name() == b"GEOM")
            .map(|chunk| unsafe { &*(chunk.bytes().as_ptr() as *const ModelFileGeometryV1) })
        else {
            return Err(std::io::ErrorKind::InvalidData.into());
        };

        let mut world = World::new();

        let material = load_context.add_labeled_asset("material", StandardMaterial::default());

        for (i, mesh) in unsafe { file_geometry.meshes.as_slice() }
            .iter()
            .enumerate()
        {
            let mesh = unsafe { mesh.as_ref() }.unwrap();

            world.spawn((
                Mesh3d(load_context.add_labeled_asset(format!("mesh_{i}"), load_mesh(mesh)?)),
                MeshMaterial3d(material.clone()),
            ));
        }

        return Ok(WorldAsset::new(world));
    }

    fn extensions(&self) -> &[&str] {
        &["model"]
    }
}

fn load_mesh(mesh: &ModelMeshDataV66) -> std::io::Result<Mesh> {
    let mesh_geometry = unsafe { mesh.geometry.as_ref() }.unwrap();

    let vertex_count = mesh_geometry.verts.vertexCount.get() as usize;

    let fvf = Fvf::from_bits_retain(mesh_geometry.verts.mesh.fvf.get());
    let texcoord_mask = (fvf & Fvf::TEXCOORD).bits() >> 8;
    let texcoord_f16_mask = (fvf & Fvf::TEXCOORD_F16).bits() >> 16;

    if fvf.contains(Fvf::POSITION | Fvf::POSITION_F16)
        || fvf.contains(Fvf::TANGENT) != fvf.contains(Fvf::BITANGENT)
        || fvf.contains(Fvf::TANGENT_FRAME)
            && fvf.intersects(Fvf::NORMAL | Fvf::TANGENT | Fvf::BITANGENT)
        || texcoord_mask & texcoord_f16_mask != 0
    {
        return Err(std::io::ErrorKind::InvalidData.into());
    }

    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = fvf
        .contains(Fvf::NORMAL)
        .then(|| Vec::with_capacity(vertex_count));
    let mut texcoords: [Option<Vec<[f32; 2]>>; 8] = std::array::from_fn(|channel| {
        ((texcoord_mask | texcoord_f16_mask) & 1 << channel != 0)
            .then(|| Vec::with_capacity(vertex_count))
    });

    let mut data = unsafe { mesh_geometry.verts.mesh.vertices.as_slice() };
    for _ in 0..vertex_count {
        if fvf.contains(Fvf::POSITION) {
            positions.push(coord::position([
                data.read_le::<f32>()?,
                data.read_le::<f32>()?,
                data.read_le::<f32>()?,
            ]));
        }

        if fvf.contains(Fvf::NORMAL) {
            normals.as_mut().unwrap().push(coord::direction([
                data.read_le::<f32>()?,
                data.read_le::<f32>()?,
                data.read_le::<f32>()?,
            ]));
        }

        if fvf.contains(Fvf::WEIGHT) {
            data = &data[4..];
        }

        if fvf.contains(Fvf::GROUP) {
            data = &data[4..];
        }

        if fvf.contains(Fvf::COLOR) {
            data = &data[4..];
        }

        if fvf.contains(Fvf::TANGENT) {
            data = &data[12..];
        }

        if fvf.contains(Fvf::BITANGENT) {
            data = &data[12..];
        }

        if fvf.contains(Fvf::TANGENT_FRAME) {
            data = &data[12..];
        }

        for (channel, texcoord) in texcoords.iter_mut().enumerate() {
            if texcoord_mask & (1 << channel) != 0 {
                #[rustfmt::skip]
                texcoord.as_mut().unwrap().push([
                    data.read_le::<f32>()?,
                    data.read_le::<f32>()?
                ]);
            }
        }

        for (channel, texcoord) in texcoords.iter_mut().enumerate() {
            if texcoord_f16_mask & (1 << channel) != 0 {
                texcoord.as_mut().unwrap().push([
                    f16::from_bits(data.read_le::<u16>()?) as f32,
                    f16::from_bits(data.read_le::<u16>()?) as f32,
                ]);
            }
        }

        if fvf.contains(Fvf::UNKNOWN_24) {
            data = &data[48..];
        }

        if fvf.contains(Fvf::UNKNOWN_25) {
            data = &data[4..];
        }

        if fvf.contains(Fvf::UNKNOWN_26) {
            data = &data[4..];
        }

        if fvf.contains(Fvf::UNKNOWN_27) {
            data = &data[16..];
        }

        if fvf.contains(Fvf::POSITION_F16) {
            positions.push(coord::position([
                f16::from_bits(data.read_le::<u16>()?) as f32,
                f16::from_bits(data.read_le::<u16>()?) as f32,
                f16::from_bits(data.read_le::<u16>()?) as f32,
            ]));
        }

        if fvf.contains(Fvf::UNKNOWN_29) {
            data = &data[12..];
        }
    }

    let mut mesh = Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(positions),
    )
    .with_inserted_indices(Indices::U16(
        unsafe { mesh_geometry.indices.indices.as_slice() }
            .chunks_exact(3)
            .flat_map(|triangle| [triangle[0].get(), triangle[2].get(), triangle[1].get()])
            .collect(),
    ));

    if let Some(normals) = normals {
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(normals),
        );
    } else {
        mesh.compute_normals();
    }

    if let Some(texcoord_0) = texcoords[0].take() {
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            VertexAttributeValues::Float32x2(texcoord_0),
        );
    }

    if let Some(texcoord_1) = texcoords[1].take() {
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_1,
            VertexAttributeValues::Float32x2(texcoord_1),
        );
    }

    Ok(mesh)
}

bitflags! {
    #[derive(Clone, Copy)]
    struct Fvf: u32 {
        const POSITION      = 1 << 0;
        const WEIGHT        = 1 << 1;
        const GROUP         = 1 << 2;
        const NORMAL        = 1 << 3;
        const COLOR         = 1 << 4;
        const TANGENT       = 1 << 5;
        const BITANGENT     = 1 << 6;
        const TANGENT_FRAME = 1 << 7;
        const TEXCOORD      = 0x7F << 8;
        const TEXCOORD_F16  = 0x7F << 16;
        const UNKNOWN_24    = 1 << 24;
        const UNKNOWN_25    = 1 << 25;
        const UNKNOWN_26    = 1 << 26;
        const UNKNOWN_27    = 1 << 27;
        const POSITION_F16  = 1 << 28;
        const UNKNOWN_29    = 1 << 29;
    }
}
