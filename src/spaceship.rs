use bevy::app::{App, Plugin, Startup};
use bevy::asset::{AssetServer, ErasedAssetLoader};
use bevy::ecs::component::{ComponentId, ComponentInfo};
use bevy::gltf::{Gltf, GltfExtras};
use bevy::math::{DVec3, Vec3};
use bevy::prelude::*;
use bevy::scene::{SceneBundle, SceneInstance};
use bevy::utils::default;
use bevy::utils::tracing::instrument::WithSubscriber;
use bevy_xpbd_3d::prelude::Position;
use crate::bevy_stupid::debug_print_components_to_console;
use crate::gentity::plugin::GEntityBundle;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_spaceship)
            .add_systems(Update, (move_spaceship_along_x_axis));
    }
}

#[derive(Component)]
pub struct Spaceship;

pub fn setup_spaceship(
    mut commands: Commands,
    mut floating_origin_settings: Res<big_space::FloatingOriginSettings>,
    asset_server: Res<AssetServer>,
) {
    // Transparency: https://github.com/bevyengine/bevy/discussions/8533
    // ToDo: Export Shader to glsl, then convert to wgsl and add it to the gltf file.
    let (grid_cell, translation) =
        floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, 149.6e9 - 10.0));
    commands.spawn((
        GEntityBundle {
            transform: Transform::from_translation(translation),
            toml: asset_server.load("spaceship\\config.toml"),
            grid_cell,
            ..default()
        },
        Spaceship,
    ));
}

pub fn move_spaceship_along_x_axis(
    time: Res<Time>,
    mut query: Query<&mut Position, With<Spaceship>>
) {
    for (mut transform) in query.iter_mut() {
        transform.z -= time.delta_seconds_f64() * 0.25;
    }
}
pub fn processs_gentity_gltf_scene(
    unloaded_instances: Query<(Entity, &SceneInstance), With<Spaceship>>,
    scene_manager: Res<SceneSpawner>,
    world: &World,
    mut cmds: Commands,
) {
    for (entity, instance) in unloaded_instances.iter() {
        if scene_manager.instance_is_ready(**instance) {
            cmds.entity(entity).remove::<Spaceship>();
        }
        let entities = scene_manager
            .iter_instance_entities(**instance)
            .chain(std::iter::once(entity));
        debug_print_components_to_console(world, entity);
        for entity_ref in entities.filter_map(|e| world.get_entity(e)) {
            debug_print_components_to_console(world, entity_ref.id());
        }
    }
}