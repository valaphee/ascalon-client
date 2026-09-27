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
    ))
    .add_systems(Startup, setup);

    #[cfg(feature = "dev")]
    app.add_plugins(bevy::camera_controller::free_camera::FreeCameraPlugin);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        #[cfg(feature = "dev")]
        bevy::camera_controller::free_camera::FreeCamera::default(),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, -1.0, 0.0)),
    ));
}
