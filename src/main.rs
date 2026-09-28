#![feature(f16, read_array, read_le)]
#![allow(dead_code)]

use bevy::prelude::*;

use crate::asset::{AssetLoaderPlugin, AssetSourcePlugin};
use crate::content::{Content, ContentPlugin};
use crate::strings::{Strings, StringsPlugin};

mod asset;
mod content;
mod strings;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        AssetSourcePlugin,
        DefaultPlugins,
        AssetLoaderPlugin,
        ContentPlugin,
        StringsPlugin,
        bevy::camera_controller::free_camera::FreeCameraPlugin,
    ))
    .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        bevy::camera_controller::free_camera::FreeCamera::default(),
    ));
}
