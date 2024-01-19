use bevy::app::{App, Plugin, Startup};
use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use bevy::input::{Axis, Input};
use bevy::math::{DVec3, Vec3};
use bevy::prelude::*;
use bevy::scene::SceneBundle;
use bevy::transform::TransformBundle;
use bevy::utils::default;
use bevy_xpbd_3d::components::{Collider, RigidBody};
use bevy_xpbd_3d::math::{Scalar, Vector2};
use bevy_xpbd_3d::prelude::{LinearVelocity, Rotation};
use big_space::camera::CameraController;
use big_space::FloatingOrigin;
use crate::bevy_stupid::vec3_to_dvec3;
use crate::camera::Camera;
use crate::player::{MovementAction, setup_player};


pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, apply_gravity)
        ;
    }
}

#[derive(Component, Default)]
pub struct GravitySource {
    pub strength: f32,
}

/// Applies [`ControllerGravity`] to character controllers.
fn apply_gravity(
    time: Res<Time>,
    gravity_sources: Query<(&GravitySource, &Transform, &Children)>,
    mut child: Query<&mut LinearVelocity>,
) {
    let delta_time = time.delta_seconds();

    for (gravity_source, transform, children) in gravity_sources.iter() {
        for &child_entity in children.iter() {
            let mut velocity = child.get_mut(child_entity).unwrap();
            let direction = transform.rotation.mul_vec3(Vec3::new(0.0, gravity_source.strength, 0.0));

            let gravity = direction * gravity_source.strength;
            velocity.0 += vec3_to_dvec3(gravity * delta_time);
        }
    }
}