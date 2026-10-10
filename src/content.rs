use std::ops::Deref;

use ascalon_asset::packfile::cntc::PackContent;
use bevy::app::{App, Plugin, Update};
use bevy::asset::{AssetServer, Assets, Handle, VisitAssetDependencies};
use bevy::ecs::resource::Resource;
use bevy::ecs::schedule::IntoScheduleConfigs as _;
use bevy::ecs::schedule::common_conditions::{not, resource_exists};
use bevy::ecs::system::{Commands, Res};
use bevy::ecs::world::{FromWorld, World};

use crate::asset::Packfile;

pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ContentHandles>().add_systems(
            Update,
            load_content
                .run_if(not(resource_exists::<ContentServer>))
                .run_if(content_dependencies_loaded),
        );
    }
}

#[derive(Resource, VisitAssetDependencies)]
pub struct ContentHandles(#[dependency] Vec<Handle<Packfile>>);

impl FromWorld for ContentHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self(
            [
                "1282830", "1282831", "1282832", "1282833", "1282834", "1282835", "1282836",
                "1282837", "1282838", "1282839", "1282840", "1282841", "1282842", "1282843",
                "1282844", "1282845", "1282846", "1282847", "1282848", "1282849", "1282850",
                "1282851", "1282852", "1282853", "1282854", "1282855", "1282856", "1282857",
                "1282858", "1282859", "1282860", "1282861",
            ]
            .into_iter()
            .map(|path| asset_server.load(path))
            .collect(),
        )
    }
}

pub fn content_dependencies_loaded(
    asset_server: Res<AssetServer>,
    handles: Res<ContentHandles>,
) -> bool {
    asset_server.are_dependencies_loaded(&*handles)
}

#[derive(Resource)]
pub struct ContentServer(ascalon_asset::content::ContentServer);

unsafe impl Send for ContentServer {}
unsafe impl Sync for ContentServer {}

impl Deref for ContentServer {
    type Target = ascalon_asset::content::ContentServer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn load_content(
    mut commands: Commands,
    assets: Res<Assets<Packfile>>,
    handles: Res<ContentHandles>,
) {
    commands.insert_resource(ContentServer(ascalon_asset::content::ContentServer::new(
        &handles
            .0
            .iter()
            .map(|handle| {
                let asset = &assets.get(handle).unwrap();
                assert_eq!(asset.r#type(), *b"cntc");

                let Some(content) = asset
                    .chunks()
                    .find(|chunk| chunk.name() == *b"Main")
                    .map(|chunk| unsafe { &*(chunk.bytes().as_ptr() as *const PackContent) })
                else {
                    panic!()
                };

                content
            })
            .collect::<Vec<_>>(),
    )));
}
