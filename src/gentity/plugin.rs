use bevy::app::{App, Plugin};
use bevy::prelude::*;
use crate::gentity::gltf::hook::*;
use crate::gentity::gltf::pp_collision::*;
use crate::gentity::gltf::pp_trigger::*;


pub struct GEntityPlugin;

impl Plugin for GEntityPlugin {
    fn build(&self, app: &mut App) {
        app
            // Hook
            .insert_resource(GEntityMap::new())
            .add_systems(Update, processs_gentity_gltf_scene.run_if(any_with_component::<ProcessGEntity>()))
            // pp_collision
            .add_systems(Startup, setup_pp_collision)
            // pp_trigger
            .add_systems(Startup, setup_pp_trigger)
            .add_systems(Update, print_collisions)
        ;
    }
}

