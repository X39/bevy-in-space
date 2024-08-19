use std::time::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_xpbd_3d::prelude::*;
use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};

#[derive(Default)]
pub struct XpbdExamplePlugin;

impl Plugin for XpbdExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PhysicsPlugins::default(), FrameTimeDiagnosticsPlugin))
            .add_state::<AppState>()
            .add_systems(Startup, setup)
            .add_systems(
                OnEnter(AppState::Paused),
                |mut time: ResMut<Time<Physics>>| time.pause(),
            )
            .add_systems(
                OnExit(AppState::Paused),
                |mut time: ResMut<Time<Physics>>| time.unpause(),
            )
            .add_systems(Update, update_fps_text)
            .add_systems(Update, pause_button)
            .add_systems(Update, step_button.run_if(in_state(AppState::Paused)));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    Paused,
    #[default]
    Running,
}

fn pause_button(
    current_state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::P) {
        let new_state = match current_state.get() {
            AppState::Paused => AppState::Running,
            AppState::Running => AppState::Paused,
        };
        next_state.0 = Some(new_state);
    }
}

fn step_button(mut time: ResMut<Time<Physics>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Return) {
        time.advance_by(Duration::from_secs_f64(1.0 / 60.0));
    }
}

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "FPS: ",
            TextStyle {
                font: default(),
                font_size: 20.0,
                color: Color::TOMATO,
            },
        )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            }),
        FpsText,
    ));
}

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[0].value = format!("FPS: {value:.2}");
            }
        }
    }
}

pub(crate) fn main() {
    App::new()
        .add_plugins((DefaultPlugins, XpbdExamplePlugin))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .insert_resource(Msaa::Sample4)
        .insert_resource(SubstepCount(50))
        .add_systems(Startup, setup2)
        .run();
}

fn setup2(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let cube_material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());

    // Kinematic rotating "anchor" object
    let anchor = commands
        .spawn((
            PbrBundle {
                mesh: cube_mesh.clone(),
                material: cube_material.clone(),
                transform: Transform::from_xyz(3.0, 3.5, 0.0),
                ..default()
            },
            RigidBody::Kinematic,
            AngularVelocity(Vector::Z * 1.5),
        ))
        .id();

    // Dynamic object rotating around anchor
    let object = commands
        .spawn((
            PbrBundle {
                mesh: cube_mesh,
                material: cube_material,
                transform: Transform::from_xyz(1.5, 0.0, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::cuboid(1.0, 1.0, 1.0), 1.0),
        ))
        .id();
    // Connect anchor and dynamic object
    commands.spawn(FixedJoint::new(anchor, object).with_local_anchor_1(Vector::X * 1.5));

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 20_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default().looking_at(Vec3::new(-1.0, -2.5, -1.5), Vec3::Y),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::Z * 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}