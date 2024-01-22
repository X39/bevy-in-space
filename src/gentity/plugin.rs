use bevy::app::{App, Plugin};
use bevy::prelude::*;
use crate::gentity::gltf::hook::*;


pub struct GEntityPlugin;

impl Plugin for GEntityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, setup_gltf_scene.run_if(any_with_component::<ProcessGEntity>()));
    }
}

