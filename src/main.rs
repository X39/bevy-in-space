mod solarsystem;
mod physics_math;
mod common_math;
mod bevy_stupid;
mod orbit;

use std::io;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::{
    prelude::*,
    transform::TransformSystem,
    window::{CursorGrabMode, PrimaryWindow, Window, WindowMode},
};
use bevy::math::DVec3;
use big_space::{
    camera::{CameraController, CameraInput},
    FloatingOrigin, GridCell,
};

fn main() {
    // simple_logging::log_to_file("log.txt", log::LevelFilter::Info);


    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<TransformPlugin>(),
            big_space::FloatingOriginPlugin::<i64>::default(),
            big_space::debug::FloatingOriginDebugPlugin::<i64>::default(),
            big_space::camera::CameraControllerPlugin::<i64>::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            solarsystem::PlanetsPlugin
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (
            setup_graphics,
            setup_physics,
            ui_setup,
        ))
        .add_systems(Update, (cursor_grab_system, ui_text_system))
        .add_systems(Main, (
            print_ball_altitude
        ))
        .run();
}

#[derive(Component, Reflect)]
pub struct BigSpaceDebugText;

fn ui_setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 28.0,
                color: Color::WHITE,
                ..default()
            },
        )
            .with_text_alignment(TextAlignment::Left)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            }),
        BigSpaceDebugText,
    ));
}

fn ui_text_system(
    mut debug_text: Query<&mut Text, (With<BigSpaceDebugText>)>,
    time: Res<Time>,
    origin: Query<(&GridCell<i64>, &Transform), With<FloatingOrigin>>,
    camera: Query<&CameraController>,
    objects: Query<&Transform, With<Handle<Mesh>>>,
) {
    let (cell, transform) = origin.single();
    let translation = transform.translation;

    let grid_text = format!("GridCell: {}x, {}y, {}z", cell.x, cell.y, cell.z);

    let translation_text = format!(
        "Transform: {:>8.2}x, {:>8.2}y, {:>8.2}z",
        translation.x, translation.y, translation.z
    );

    let velocity = camera.single().velocity();
    let speed = velocity.0.length() / time.delta_seconds_f64();
    let camera_text = if speed > 3.0e8 {
        format!("Speed: {:.0e} * speed of light", speed / 3.0e8)
    } else {
        format!("Speed: {:.2e} m/s", speed)
    };

    debug_text.single_mut().sections[0].value =
        format!("{grid_text}\n{translation_text}\n{camera_text}");
}

fn cursor_grab_system(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut cam: ResMut<CameraInput>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let Some(mut window) = windows.get_single_mut().ok() else {
        return;
    };

    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        window.mode = WindowMode::BorderlessFullscreen;
        cam.defaults_disabled = false;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        window.mode = WindowMode::Windowed;
        cam.defaults_disabled = true;
    }
}

fn setup_graphics(
    mut floating_origin_settings: Res<big_space::FloatingOriginSettings>,
    mut commands: Commands,
) {
    let (grid_cell, translation) = floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, 149.6e9));
    // camera
    commands.spawn((
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

fn setup_physics(mut commands: Commands) {
    // /* Create the ground. */
    // commands
    //     .spawn(Collider::cuboid(100.0, 0.1, 100.0))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));
    //
    // /* Create the bouncing ball. */
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Collider::ball(0.5))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    // for transform in positions.iter() {
    //     println!("Ball altitude: {}", transform.translation.y);
    // }
}