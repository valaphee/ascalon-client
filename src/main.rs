#![feature(f16, read_array, read_le)]

use std::{ffi::OsString, os::windows::ffi::OsStringExt, path::PathBuf};

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
    .add_systems(
        Update,
        (
            debug.run_if(resource_added::<Content>),
            //debug_2.run_if(resource_exists::<Content>),
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

fn debug(mut commands: Commands, asset_server: Res<AssetServer>, content: Res<Content>) {
    let item = content.get::<Item>(14065).unwrap();
    info!("{item:#?}");

    let map = content.get::<Map>(573).unwrap();
    info!("{map:#?}");

    commands.spawn((
        ImageNode::new(asset_server.load(PathBuf::from(OsString::from_wide(unsafe {
            item.icon.as_slice()
        })))),
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

fn debug_2(strings: Strings, content: Res<Content>) {
    let map = content.get::<Map>(38).unwrap();
    info!(
        "{} {}",
        strings.get(map._168).unwrap_or_default(),
        strings.get(map._16c).unwrap_or_default(),
    );
}
