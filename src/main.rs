#![feature(f16, read_array, read_le)]

use bevy::prelude::*;

use crate::{
    asset::{AssetLoaderPlugin, AssetSourcePlugin},
    content::{Content, ContentPlugin, Item},
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
    .add_systems(
        Update,
        (
            debug_item.run_if(resource_added::<Content>),
            debug_item_2.run_if(resource_exists::<Content>),
        ),
    );

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

fn debug_item(mut commands: Commands, asset_server: Res<AssetServer>, content: Res<Content>) {
    let item = content.get::<Item>(4925).unwrap();
    info!("{item:?}");

    commands.spawn((
        ImageNode::new(asset_server.load(unsafe { item.icon.to_string_lossy() })),
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

fn debug_item_2(strings: Strings, content: Res<Content>) {
    let item = content.get::<Item>(4925).unwrap();
    info!(
        "{} {}",
        strings.get(item.name.get()).unwrap_or_default(),
        strings.get(item.description.get()).unwrap_or_default(),
    );
}
