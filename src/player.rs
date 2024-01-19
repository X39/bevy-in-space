use bevy::math::DVec3;
use big_space::FloatingOrigin;
use bevy::prelude::*;
use bevy_xpbd_3d::math::{Quaternion, Scalar, Vector, Vector2};
use bevy_xpbd_3d::prelude::*;
use bevy_xpbd_3d::{SubstepSchedule, SubstepSet};


pub struct PlayerPlugin;

const max_slope_angle_f64: f64 = 30.0;
const max_slope_angle_f32: f32 = 30.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MovementAction>()
            .add_systems(Startup, setup_player)
            .add_systems(
                Update,
                (
                    keyboard_input,
                    gamepad_input,
                    update_grounded,
                    apply_deferred,
                    movement,
                    apply_movement_damping,
                )
                    .chain(),
            )
            .add_systems(
                // Run collision handling in substep schedule
                SubstepSchedule,
                kinematic_controller_collisions.in_set(SubstepSet::SolveUserConstraints),
            );
        ;
    }
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Event)]
pub enum MovementAction {
    Move(Vector2),
    Jump,
}

#[derive(Component)]
pub struct MovementAcceleration(Scalar);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(Scalar);

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
    let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
    let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vector2::new(horizontal as Scalar, vertical as Scalar).clamp_length_max(1.0);

    if direction != Vector2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement_event_writer.send(MovementAction::Jump);
    }
}

/// Sends [`MovementAction`] events based on gamepad input.
fn gamepad_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
) {
    for gamepad in gamepads.iter() {
        let axis_lx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX,
        };
        let axis_ly = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickY,
        };

        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            movement_event_writer.send(MovementAction::Move(
                Vector2::new(x as Scalar, y as Scalar).clamp_length_max(1.0),
            ));
        }

        let jump_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South,
        };

        if buttons.just_pressed(jump_button) {
            movement_event_writer.send(MovementAction::Jump);
        }
    }
}

pub fn setup_player(
    mut commands: Commands,
    mut floating_origin_settings: Res<big_space::FloatingOriginSettings>,
    asset_server: Res<AssetServer>,
) {
    // ToDo: Spawn a player with xpbd physics
    // ToDo: Attach camera to player and make it follow the player
    let (grid_cell, translation) = floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, 149.6e9));
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(100.0, 0.1, 100.0),
        grid_cell,
        TransformBundle::from(Transform::from_translation(translation - Vec3::Y * 5.0))
    ));
    commands.spawn((
        RigidBody::Kinematic,
        LinearVelocity::from(DVec3::new(0.0, -1.0, 0.0)),
        Collider::capsule(1.72, 0.1),
        grid_cell,
        TransformBundle::from(Transform::from_translation(translation + Vec3::Y * 1.0)),
        Player,
        FloatingOrigin,
        MovementAcceleration(30.0),
        MovementDampingFactor(0.92),
        JumpImpulse(7.0),
        ShapeCaster::new(
            Collider::capsule(1.72, 0.1),
            Vector::ZERO,
            Quaternion::default(),
            Vector::NEG_Y,
        )
    )).with_children(|parent| {
        parent.spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.7, 5.0)),
            ..default()
        });
    });
}

fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation),
        With<Player>,
    >,
) {
    for (entity, hits, rotation) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            hit.time_of_impact < 0.0001 && rotation.rotate(-hit.normal2).angle_between(Vector::Y).abs() <= max_slope_angle_f64
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}

fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        Has<Grounded>,
    )>,
) {
    let delta_time = time.delta_seconds_f64();

    for event in movement_event_reader.read() {
        for (movement_acceleration, jump_impulse, mut linear_velocity, is_grounded) in
        &mut controllers
        {
            if is_grounded {
                match event {
                    MovementAction::Move(direction) => {
                        linear_velocity.x += direction.x * movement_acceleration.0 * delta_time;
                        linear_velocity.z -= direction.y * movement_acceleration.0 * delta_time;
                    }
                    MovementAction::Jump => {
                        linear_velocity.y = jump_impulse.0;
                    }
                }
            }
        }
    }
}

/// Kinematic bodies do not get pushed by collisions by default,
/// so it needs to be done manually.
///
/// This system performs very basic collision response for kinematic
/// character controllers by pushing them along their contact normals
/// by the current penetration depths.
#[allow(clippy::type_complexity)]
fn kinematic_controller_collisions(
    collisions: Res<Collisions>,
    collider_parents: Query<&ColliderParent, Without<Sensor>>,
    mut character_controllers: Query<
        (
            &RigidBody,
            &mut Position,
            &Rotation,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // If the collision didn't happen during this substep, skip the collision
        if !contacts.during_current_substep {
            continue;
        }

        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([collider_parent1, collider_parent2]) =
            collider_parents.get_many([contacts.entity1, contacts.entity2])
            else {
                continue;
            };

        // Get the body of the character controller and whether it is the first
        // or second entity in the collision.
        let is_first: bool;
        let (rb, mut position, rotation, mut linear_velocity) =
            if let Ok(character) = character_controllers.get_mut(collider_parent1.get()) {
                is_first = true;
                character
            } else if let Ok(character) = character_controllers.get_mut(collider_parent2.get()) {
                is_first = false;
                character
            } else {
                continue;
            };

        // This system only handles collision response for kinematic character controllers
        if !rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.global_normal1(rotation)
            } else {
                -manifold.global_normal2(rotation)
            };

            // Solve each penetrating contact in the manifold
            for contact in manifold.contacts.iter().filter(|c| c.penetration > 0.0) {
                position.0 += normal * contact.penetration;
            }

            // If the slope isn't too steep to walk on but the character
            // is falling, reset vertical velocity.
            if normal.angle_between(Vector::Y).abs() <= max_slope_angle_f64 && linear_velocity.y < 0.0
            {
                linear_velocity.y = linear_velocity.y.max(0.0);
            }
        }
    }
}