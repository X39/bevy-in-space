use bevy::app::{App, Plugin};
use bevy::prelude::*;
use big_space::GridCell;
use crate::gentity::asset_loaders;
use crate::gentity::gltf::hook::*;
use crate::gentity::gltf::pp_collision::*;
use crate::gentity::gltf::pp_trigger::*;
use crate::gentity::asset_loaders::toml_asset_loader::*;


#[derive(Default, Component)]
pub struct GEntityInitializeFromTomlComponent;

#[derive(Default, Bundle)]
pub struct GEntityBundle {
    pub toml: Handle<TomlAsset>,
    pub transform: Transform,
    pub grid_cell: GridCell<i64>,
    pub init: GEntityInitializeFromTomlComponent,
}
pub struct GEntityPlugin;

impl Plugin for GEntityPlugin {
    fn build(&self, app: &mut App) {
        app
            // Hook
            .insert_resource(GEntityMap::new())
            .add_plugins((
                asset_loaders::rhai_asset_loader::Plugin,
                asset_loaders::toml_asset_loader::Plugin,
                ))
            .add_systems(Update, processs_gentity_gltf_scene.run_if(any_with_component::<ProcessGEntity>()))
            .add_systems(Update, process_gentity_toml_file.run_if(any_with_component::<GEntityInitializeFromTomlComponent>()))
            // pp_collision
            .add_systems(Startup, setup_pp_collision)
            // pp_trigger
            .add_systems(Startup, setup_pp_trigger)
            .add_systems(Update, print_collisions)
        ;
    }
}

