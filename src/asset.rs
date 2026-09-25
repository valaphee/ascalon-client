use std::{io::Read as _, os::windows::ffi::OsStrExt as _, path::Path};

use ascalon_asset::{
    archive::Archive,
    file_name_to_id,
    texture::{FF_ALPHA, FF_BICOLOR, FF_COLOR, FF_DEDUCED_ALPHA, FF_PLAIN, inflate},
};
use bevy::{
    app::{App, Plugin},
    asset::{
        Asset, AssetApp, AssetLoader, LoadContext, RenderAssetUsages,
        io::{AssetReaderError, AssetSourceBuilder, AssetSourceId, PathStream, Reader, VecReader},
    },
    image::Image,
    mesh::Mesh,
    reflect::TypePath,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

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
        let mut file_name = path.as_os_str().encode_wide();
        let file_id =
            file_name_to_id(&[file_name.next().unwrap(), file_name.next().unwrap()]).unwrap();

        Ok(VecReader::new(self.0.read(file_id)?))
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
        app.init_asset::<Packfile>()
            .init_asset_loader::<PackfileLoader>()
            .init_asset_loader::<ImageLoader>()
            .init_asset_loader::<ModelLoader>();
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
        let bytes = std::mem::take(&mut unsafe { assume_vec_reader(reader) }.bytes);

        Ok(Packfile(ascalon_asset::packfile::Packfile::new(bytes)?))
    }
}

#[derive(Default, TypePath)]
struct ImageLoader;

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
        let bytes = std::mem::take(&mut unsafe { assume_vec_reader(reader) }.bytes);

        let mut bytes = bytes.as_slice();
        let magic = bytes.read_array::<4>()?;
        if !matches!(
            &magic,
            b"ATEX" | b"ATTX" | b"ATEC" | b"ATEP" | b"ATET" | b"ATEU"
        ) {
            return Err(std::io::ErrorKind::InvalidData.into());
        }

        let format = bytes.read_array::<4>()?;
        let (format, format_flags) = match &format {
            b"DXT1" => (
                TextureFormat::Bc1RgbaUnorm,
                FF_COLOR | FF_ALPHA | FF_DEDUCED_ALPHA,
            ),
            b"DXT2" | b"DXT3" => (TextureFormat::Bc2RgbaUnorm, FF_COLOR | FF_ALPHA | FF_PLAIN),
            b"DXT4" | b"DXT5" => (TextureFormat::Bc3RgbaUnorm, FF_COLOR | FF_ALPHA | FF_PLAIN),
            b"DXTA" => (TextureFormat::Bc4RUnorm, FF_ALPHA | FF_PLAIN),
            b"DXTN" | b"3DCX" | b"BC5X" => (TextureFormat::Bc5RgUnorm, FF_BICOLOR),
            b"BC7X" => (TextureFormat::Bc7RgbaUnorm, FF_COLOR | FF_ALPHA | FF_PLAIN),
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

#[derive(Default, TypePath)]
struct ModelLoader;

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
        let _bytes = std::mem::take(&mut unsafe { assume_vec_reader(reader) }.bytes);

        return Err(std::io::ErrorKind::InvalidData.into());
    }
}

unsafe fn assume_vec_reader(reader: &mut dyn Reader) -> &mut VecReader {
    unsafe { &mut *(reader as *mut dyn Reader as *mut VecReader) }
}
