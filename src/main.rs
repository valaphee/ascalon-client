#![feature(read_array, read_le)]

use bevy::prelude::*;

use crate::asset::{AssetLoaderPlugin, AssetSourcePlugin};

mod asset;

fn main() {
    App::new()
        .add_plugins((AssetSourcePlugin, DefaultPlugins, AssetLoaderPlugin))
        .run();
}
