#![feature(read_array, read_le)]

use bevy::prelude::*;

use crate::{
    asset::{AssetLoaderPlugin, AssetSourcePlugin},
    content::ContentPlugin,
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
        ))
        .run();
}
