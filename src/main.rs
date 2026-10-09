#![feature(f16, read_array, read_le)]
#![allow(dead_code)]

use bevy::prelude::*;

use crate::asset::{AssetLoaderPlugin, AssetSourcePlugin};
use crate::content::{ContentContext, ContentPlugin};
use crate::render::RenderPlugin;
use crate::text::TextPlugin;

mod asset;
mod content;
mod render;
mod text;
mod unit;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        AssetSourcePlugin,
        DefaultPlugins,
        AssetLoaderPlugin,
        ContentPlugin,
        TextPlugin,
        RenderPlugin,
        bevy::camera_controller::free_camera::FreeCameraPlugin,
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, debug.run_if(resource_added::<ContentContext>));

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        bevy::camera_controller::free_camera::FreeCamera::default(),
    ));
}

fn debug(
    mut commands: Commands,
    content_context: Res<ContentContext>,
    asset_server: Res<AssetServer>,
) {
    let map = content_context.by_data_id::<content::Map>(22).unwrap();

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
