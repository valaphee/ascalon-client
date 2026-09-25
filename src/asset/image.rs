use std::io::Read as _;

use ascalon_asset::texture::{
    FMT_ALPHA, FMT_BICOLOR, FMT_COLOR, FMT_DEDUCED_ALPHA, FMT_PLAIN, inflate,
};
use bevy::{
    asset::{
        AssetLoader, LoadContext, RenderAssetUsages,
        io::{Reader, VecReader},
    },
    image::Image,
    reflect::TypePath,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

#[derive(Default, TypePath)]
pub struct ImageLoader;

impl AssetLoader for ImageLoader {
    type Asset = Image;
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
        if !matches!(
            &magic,
            b"ATEX" | b"ATTX" | b"ATEC" | b"ATEP" | b"ATET" | b"ATEU"
        ) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "image: invalid magic",
            ));
        }

        let format = bytes.read_array::<4>()?;
        let (format, format_flags) = match &format {
            b"DXT1" => (
                TextureFormat::Bc1RgbaUnorm,
                FMT_COLOR | FMT_ALPHA | FMT_DEDUCED_ALPHA,
            ),
            b"DXT2" | b"DXT3" => (
                TextureFormat::Bc2RgbaUnorm,
                FMT_COLOR | FMT_ALPHA | FMT_PLAIN,
            ),
            b"DXT4" | b"DXT5" => (
                TextureFormat::Bc3RgbaUnorm,
                FMT_COLOR | FMT_ALPHA | FMT_PLAIN,
            ),
            b"DXTA" => (TextureFormat::Bc4RUnorm, FMT_ALPHA | FMT_PLAIN),
            b"DXTN" | b"3DCX" | b"BC5X" => (TextureFormat::Bc5RgUnorm, FMT_BICOLOR),
            b"BC7X" => (
                TextureFormat::Bc7RgbaUnorm,
                FMT_COLOR | FMT_ALPHA | FMT_PLAIN,
            ),
            _ => return Err(std::io::ErrorKind::InvalidData.into()),
        };

        let (block_width, block_height) = format.block_dimensions();
        let block_size = format
            .block_copy_size(None)
            .ok_or(std::io::ErrorKind::InvalidData)? as usize;

        let width = bytes.read_le::<u16>()? as u32;
        let height = bytes.read_le::<u16>()? as u32;

        let mut data = Vec::new();
        let mut mip_level_count = 0;

        while bytes.len() >= 8 {
            let size = bytes.read_le::<u32>()? as usize;
            let size = size.checked_sub(8).ok_or(std::io::ErrorKind::InvalidData)?;
            if size > bytes.len() {
                return Err(std::io::ErrorKind::UnexpectedEof.into());
            }

            let compression_flags = bytes.read_le::<u32>()?;

            let _bytes = bytes.split_at(size);
            bytes = _bytes.1;

            let mip_width = (width >> mip_level_count).max(1);
            let mip_height = (height >> mip_level_count).max(1);
            let output_size = (mip_width.div_ceil(block_width) * mip_height.div_ceil(block_height))
                as usize
                * block_size;

            let start = data.len();
            data.resize(start + output_size, 0);

            inflate(
                block_size,
                format_flags,
                compression_flags,
                _bytes.0,
                &mut data[start..],
            )?;

            mip_level_count += 1;
        }

        let mut image = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            format,
            RenderAssetUsages::default(),
        );

        image.texture_descriptor.mip_level_count = mip_level_count;

        Ok(image)
    }
}
