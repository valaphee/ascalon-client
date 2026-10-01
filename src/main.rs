#![feature(f16, read_array, read_le)]
#![allow(dead_code)]

use bevy::prelude::*;

use crate::asset::{AssetLoaderPlugin, AssetSourcePlugin};
use crate::content::{Content, ContentPlugin};
use crate::strings::{Strings, StringsPlugin};

mod asset;
mod content;
mod coord;
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
    .add_systems(Startup, setup)
    .add_systems(Update, debug.run_if(resource_added::<Content>));

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        bevy::camera_controller::free_camera::FreeCamera::default(),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 20_000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, -0.6, 0.0)),
    ));
}

fn debug(mut commands: Commands, content: Res<Content>, asset_server: Res<AssetServer>) {
    let map = content.get::<content::Map>(350).unwrap();

    commands.spawn(WorldAssetRoot(
        asset_server.load(unsafe { map.fileMap.file_id() }.unwrap().to_string()),
    ));
}
