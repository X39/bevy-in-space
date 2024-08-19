use crate::orbit::Orbit;
use log::info;
use bevy::math::cubic_splines::Point;
use bevy::app::{App, Main, Plugin, PostUpdate, Startup, Update};
use bevy::asset::Assets;
use bevy::input::Input;
use bevy::math::{DVec3, dvec3, I64Vec3, IVec3, Vec3, Vec3A};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Camera3dBundle, Color, Commands, Component, Entity, Gizmos, GlobalTransform, Image, KeyCode, Mesh, MouseButton, Mut, PerspectiveProjection, Projection, Query, Res, ResMut, Resource, Time, Transform, Window, With, Without};
use bevy::prelude::shape::UVSphere;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::utils::default;
use bevy::window::{CursorGrabMode, PrimaryWindow, WindowMode};
use big_space::{FloatingOrigin, FloatingOriginSettings, GridCell};
use big_space::camera::{CameraController, CameraInput};
use crate::bevy_stupid::{dvec3_to_vec3, vec3_to_dvec3};
use crate::common_math::distance3_f64;
use crate::physics_math::{double, single};
use rand::Rng;


pub struct CameraPlugin;

#[derive(Default, Component)]
pub struct Camera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(big_space::camera::CameraControllerPlugin::<i64>::default())
            .add_systems(Startup, setup_camera)
            .add_systems(Update, swap_camera_by_key);
    }
}


fn swap_camera_by_key(
    mut camera_query: Query<(Entity, &mut CameraController), With<(Camera)>>,
    mut camera_input: ResMut<CameraInput>,
    key: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    let Some((entity_id, mut camera_controller)) = camera_query.get_single_mut().ok() else {
        return;
    };

    if key.just_pressed(KeyCode::Key1) {
        camera_controller.speed = 0.0;
        camera_controller.smoothness = 0.0;
        camera_controller.rotational_smoothness = 0.0;
        camera_input.defaults_disabled = true;
    }
    if key.just_pressed(KeyCode::Key2) {
        camera_controller.speed = 1.0;
        camera_controller.smoothness = 0.9;
        camera_controller.rotational_smoothness = 0.8;
        camera_controller.speed_bounds = [10e-18, 10e35];
        camera_input.defaults_disabled = false;
    }
    if key.just_pressed(KeyCode::Key3) {
        camera_controller.speed = 0.000000001;
        camera_controller.smoothness = 0.9;
        camera_controller.rotational_smoothness = 0.8;
        camera_controller.speed_bounds = [10e-18, 1.0];
        camera_input.defaults_disabled = false;
    }
}


fn setup_camera(
    mut floating_origin_settings: Res<big_space::FloatingOriginSettings>,
    mut commands: Commands,
) {
    let (grid_cell, translation) = floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, 149.6e9));
    // camera
    commands.spawn((
        Camera,
        Camera3dBundle {
            transform: Transform::from_translation(translation)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                near: 1e-18,
                ..default()
            }),
            ..default()
        },
        grid_cell,
        FloatingOrigin,
        CameraController::default() // Built-in camera controller
            .with_speed_bounds([10e-18, 10e35])
            .with_smoothness(0.9, 0.8)
            .with_speed(1.0),
    ));
}