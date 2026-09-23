#![feature(read_array, read_le)]

use bevy::prelude::*;

use crate::{
    asset::{AssetLoaderPlugin, AssetSourcePlugin},
    content::ContentPlugin,
};

mod asset;
mod content;

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
