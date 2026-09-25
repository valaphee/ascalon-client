use bevy::asset::VisitAssetDependencies;
use bevy::prelude::*;

use crate::asset::Packfile;

pub struct StringsPlugin;

impl Plugin for StringsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StringsHandle>();
    }
}

#[derive(Resource, VisitAssetDependencies)]
pub struct StringsHandle(#[dependency] Handle<Packfile>);

impl FromWorld for StringsHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self(asset_server.load("댐ā"))
    }
}
