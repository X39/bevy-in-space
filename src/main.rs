mod solarsystem;
mod physics_math;
mod common_math;
mod bevy_stupid;
mod orbit;
mod spaceship;
mod camera;
mod player;
mod gravity;
mod gentity;
mod localization;

use std::io;

use bevy::prelude::*;
use bevy::{
    prelude::*,
    transform::TransformSystem,
    window::{CursorGrabMode, PrimaryWindow, Window, WindowMode},
};
use bevy::math::DVec3;
use bevy_xpbd_3d::plugins::{BroadPhasePlugin, ContactReportingPlugin, IntegratorPlugin, NarrowPhasePlugin, PhysicsSetupPlugin, PreparePlugin, SleepingPlugin, SolverPlugin, SpatialQueryPlugin, SyncPlugin};
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
            // big_space::debug::FloatingOriginDebugPlugin::<i64>::default(),
        ))
        .add_plugins((
            // From: bevy_xpbd_3d::plugins::PhysicsPlugins::default(),
            bevy_xpbd_3d::plugins::PhysicsSetupPlugin::new(PostUpdate),
            bevy_xpbd_3d::plugins::PreparePlugin::new(PostUpdate),
            bevy_xpbd_3d::plugins::BroadPhasePlugin,
            bevy_xpbd_3d::plugins::IntegratorPlugin,
            bevy_xpbd_3d::plugins::NarrowPhasePlugin,
            bevy_xpbd_3d::plugins::ContactReportingPlugin,
            bevy_xpbd_3d::plugins::SolverPlugin,
            bevy_xpbd_3d::plugins::SleepingPlugin,
            bevy_xpbd_3d::plugins::SpatialQueryPlugin::new(PostUpdate),
            // bevy_xpbd_3d::plugins::SyncPlugin::new(PostUpdate),
            big_space::bevy_xpbd::floating_origin_sync::FloatingOriginSyncPlugin::<i64>::new(PostUpdate),
            bevy_xpbd_3d::plugins::PhysicsDebugPlugin::default(),
            ))
        .add_plugins((
            gentity::plugin::GEntityPlugin,
            localization::LocalizationPlugin::new("en-US".to_string()),
            solarsystem::PlanetsPlugin,
            camera::CameraPlugin,
            // player::PlayerPlugin,
            spaceship::SpaceshipPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (
            ui_setup,
        ))
        .add_systems(Update, (cursor_grab_system, ui_text_system))
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

    if let Ok(single) = camera.get_single() {
        let velocity = single.velocity();
        let speed = velocity.0.length() / time.delta_seconds_f64();
        let camera_text = if speed > 3.0e8 {
            format!("Speed: {:.0e} * speed of light", speed / 3.0e8)
        } else {
            format!("Speed: {:.2e} m/s", speed)
        };
        debug_text.single_mut().sections[0].value =
            format!("{grid_text}\n{translation_text}\n{camera_text}");
    }

}

fn cursor_grab_system(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
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
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        window.mode = WindowMode::Windowed;
    }
}