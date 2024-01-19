use bevy::app::{App, Plugin, Startup};
use bevy::asset::{AssetServer, ErasedAssetLoader};
use bevy::gltf::Gltf;
use bevy::math::{DVec3, Vec3};
use bevy::prelude::*;
use bevy::scene::SceneBundle;
use bevy::utils::default;
use bevy::utils::tracing::instrument::WithSubscriber;


pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_spaceship);
    }
}


pub fn setup_spaceship(
    mut commands: Commands,
    mut floating_origin_settings: Res<big_space::FloatingOriginSettings>,
    asset_server: Res<AssetServer>,
) {
    // Transparency: https://github.com/bevyengine/bevy/discussions/8533
    // ToDo: Export Shader to glsl, then convert to wgsl and add it to the gltf file.
    let (grid_cell, translation) = floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, 149.6e9 + 10.0));
    commands.spawn((
        SceneBundle {
            transform: Transform::from_translation(translation),
            scene: asset_server.load("spaceship\\scene.gltf#Scene0"),
            ..default()
        },
        grid_cell
    ));
}