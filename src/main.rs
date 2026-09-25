#![feature(f16, read_array, read_le)]

use bevy::prelude::*;

use crate::{
    asset::{AssetLoaderPlugin, AssetSourcePlugin},
    content::ContentPlugin,
    strings::StringsPlugin,
};

mod asset;
mod content;
mod strings;

fn main() {
    App::new()
        .add_plugins((
            AssetSourcePlugin,
            DefaultPlugins,
            AssetLoaderPlugin,
            ContentPlugin,
            StringsPlugin,
        ))
        .run();
}
