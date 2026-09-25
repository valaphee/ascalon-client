#![feature(f16, read_array, read_le)]

use bevy::prelude::*;

use crate::{
    asset::{AssetLoaderPlugin, AssetSourcePlugin},
    content::{Content, ContentPlugin, Item, Map},
    strings::{Strings, StringsPlugin},
};

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
    .add_systems(Startup, setup)
    .add_systems(Update, debug.run_if(resource_added::<Content>));

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
}

fn debug(mut commands: Commands, asset_server: Res<AssetServer>, content: Res<Content>) {
    let item = content.get::<Item>(14065).unwrap();
    info!("{item:?}");

    let map = content.get::<Map>(15).unwrap();
    info!("{map:?}");

    commands.spawn((
        ImageNode::new(asset_server.load(unsafe { map._58.to_string_lossy() })),
        Node {
            position_type: PositionType::Absolute,
            left: px(16),
            bottom: px(16),
            width: px(64),
            height: px(64),
            ..default()
        },
    ));
}
