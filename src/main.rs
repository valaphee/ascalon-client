#![feature(f16, read_array, read_le)]
#![allow(dead_code)]

use bevy::prelude::*;

use crate::asset::{AssetLoaderPlugin, AssetSourcePlugin};
use crate::content::{Content, ContentPlugin};
use crate::text::TextPlugin;

mod asset;
mod content;
mod coord;
mod text;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        AssetSourcePlugin,
        DefaultPlugins,
        AssetLoaderPlugin,
        ContentPlugin,
        TextPlugin,
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
    let map = content.get::<content::MapDef>(350).unwrap();

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((text::Text::new(map.textName), Text::default()));

            parent.spawn((text::Text::new(map.textDescription), Text::default()));
        });

    commands.spawn(WorldAssetRoot(asset_server.load(format!(
        "{}.amap2c",
        unsafe { map.fileMap.file_id() }.unwrap()
    ))));
}
