use crate::orbit::Orbit;
use log::info;
use bevy::math::cubic_splines::Point;
use bevy::app::{App, Main, Plugin, PostUpdate, Startup, Update};
use bevy::asset::Assets;
use bevy::math::{DVec3, dvec3, I64Vec3, IVec3, Vec3, Vec3A};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Color, Commands, Component, Entity, Gizmos, GlobalTransform, Image, Mesh, Mut, Query, Res, ResMut, Resource, Time, Transform, With, Without};
use bevy::prelude::shape::UVSphere;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::utils::default;
use big_space::{FloatingOrigin, FloatingOriginSettings, GridCell};
use crate::bevy_stupid::{dvec3_to_vec3, vec3_to_dvec3};
use crate::common_math::distance3_f64;
use crate::physics_math::{double, single};
use rand::Rng;

pub struct PlanetsPlugin;

impl Plugin for PlanetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SimulationSpeed(86400.0))
            //.insert_resource(SimulationSpeed(1.0))
            //.insert_resource(SimulationSpeed(1000.0))
            .add_systems(Startup, (setup_planets))
            .add_systems(Update, (update_planets))
            .add_systems(PostUpdate, (post_update_planets, log_planets));
    }
}

#[derive(Component)]
pub struct MassNoEffect;
#[derive(Component)]
pub struct Sun;

#[derive(Component)]
pub struct Mass {
    /**
     * The mass of the object.
     */
    value: f32,

    /**
     * The center of mass of the object in relation to the object's origin.
     */
    center: Vec3,
}

impl Mass {
    pub fn new(mass: f32) -> Self {
        Self {
            value: mass,
            center: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub const fn zero() -> Self {
        Self {
            value: 0.0,
            center: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    /**
     * #### Description
     * Computes the acceleration of the object due to the gravitational force of another object.
     *
     * #### Remarks
     * The distance should be the relative to the origin of the object, not the center of mass.
     * The center of mass will be added to the distance before the acceleration is computed.
     *
     * #### Parameters
     * - `other` -- The other object.
     * - `distance` -- The distance between the two objects.
     *
     * #### Returns
     * The acceleration of the object due to the gravitational force of another object.
     */
    pub fn acceleration_f64(&self, other: &Mass, position_self: DVec3, position_other: DVec3) -> DVec3 {
        let distance: DVec3 = position_self - vec3_to_dvec3(self.center) - position_other - vec3_to_dvec3(other.center);
        if distance.x == 0.0 && distance.y == 0.0 && distance.z == 0.0 {
            return DVec3::new(0.0, 0.0, 0.0);
        }
        let unit: DVec3 = distance.normalize();
        let force = double::compute_gravitational_force(self.value as f64, other.value as f64, distance.length());
        let acceleration = force / self.value as f64;
        let acceleration: DVec3 = unit * acceleration;
        acceleration
    }
}

#[derive(Component, Clone, Debug, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct Name(String);

#[derive(Component, Copy, Clone, Debug, PartialEq)]
struct Velocity {
    velocity: Vec3,
}

#[derive(Resource)]
struct SimulationSpeed(f32);

impl Velocity {
    fn new(velocity: Vec3) -> Self {
        Self {
            velocity,
        }
    }
    fn zero() -> Self {
        Self {
            velocity: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    /**
     * Apply the velocity to the position, updating the sector if necessary.
     * @param position The position to update.
     */
    fn apply(&mut self, transform: &mut Transform, delta: f32) {
        let position: Vec3 = transform.translation.clone();
        let velocity: Vec3 = self.velocity;
        let new_position = position + velocity * delta;
        let delta = new_position - position;
        transform.translation += delta;
    }
}
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255,
        255, 159, 102, 255,
        236, 255, 102, 255,
        121, 255, 102, 255,
        102, 255, 198, 255,
        102, 198, 255, 255,
        121, 102, 255, 255,
        236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}

fn uv_sun_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 255, 0, 255,
        255, 255, 0, 255,
        255, 255, 0, 255,
        255, 255, 0, 255,
        255, 255, 0, 255,
        255, 255, 0, 255,
        255, 255, 0, 255,
        255, 255, 0, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}

fn update_planets(
    floating_origin_settings: Res<big_space::FloatingOriginSettings>,
    time: Res<Time>,
    simulation_speed: Res<SimulationSpeed>,
    mut query_self: Query<(Entity, &mut Velocity, &Mass, &Transform, &GridCell<i64>)>,
    mut query2: Query<(Entity, &Mass, &Transform, &GridCell<i64>), (Without<MassNoEffect>)>,
) {
    for (id_self, mut velocity_self, mass_self, transform_self, grid_cell_self) in query_self.iter_mut() {
        let mut velocity_out = velocity_self;
        for (id_other, mass_other, transform_other, grid_cell_other) in query2.iter_mut() {
            if id_self == id_other {
                continue;
            }
            let position_self = DVec3::default();
            let position_other: DVec3 = floating_origin_settings.grid_position_double::<i64>(
                &(grid_cell_other - grid_cell_self),
                transform_other,
            ) - vec3_to_dvec3(transform_self.translation);

            let mut acceleration = mass_self.acceleration_f64(mass_other, position_self, position_other);
            acceleration *= time.delta_seconds() as f64 * simulation_speed.0 as f64;
            velocity_out.velocity -= dvec3_to_vec3(acceleration);
        }
        velocity_self = velocity_out;
    }
}

fn log_planets(
    simulation_speed: Res<SimulationSpeed>,
    mut gizmos: Gizmos,
    floating_origin_settings: Res<FloatingOriginSettings>,
    time: Res<Time>,
    mut query1: Query<(Entity, &Velocity, &GridCell<i64>, &Transform, &GlobalTransform, Option<&MassNoEffect>)>,
    mut orbits: Query<(&Orbit, &GlobalTransform)>,
    mut sun: Query<(&GlobalTransform, &Transform), With<Sun>>,
) {
    // let sim_speed: f64 = (time.delta_seconds() * simulation_speed.0) as f64;
    let sim_speed: f64 = (simulation_speed.0 * 0.25) as f64;
    for (id, velocity, grid, transform, glob_transform, opt) in query1.iter_mut() {

        // let pos = floating_origin_settings.grid_position::<i64>(grid, transform);
        // gizmos.line(
        //     pos,
        //     pos + velocity.velocity * time.delta_seconds() * simulation_speed.0 * DISTANCE_SCALE,
        //     Color::TOMATO
        // );
        // gizmos.line(
        //     transform.translation,
        //     transform.translation + velocity.velocity * time.delta_seconds() * simulation_speed.0 * DISTANCE_SCALE,
        //     Color::TOMATO
        // );
        let (_, _, translation) = glob_transform.to_scale_rotation_translation();
        let distance_scale = if opt.is_some() {10.0} else {100.0};
        gizmos.line(
            translation,
            dvec3_to_vec3(vec3_to_dvec3(translation) + vec3_to_dvec3(velocity.velocity) * sim_speed * distance_scale),
            if opt.is_some() {Color::TOMATO} else {Color::CYAN}
        );
        // info!("{}: Grid: {:?}, Position: {:?}, Velocity: {:?}", id.index(), grid, transform.translation, velocity);
    }
}

fn post_update_planets(
    time: Res<Time>,
    simulation_speed: Res<SimulationSpeed>,
    mut query: Query<(&mut Velocity, &mut Transform)>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        velocity.apply(&mut transform, time.delta_seconds() * simulation_speed.0);
    }
}

fn setup_planets(
    mut commands: Commands,
    mut floating_origin_settings: Res<big_space::FloatingOriginSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sun
    let scale = Vec3::new(1.0, 1.0, 1.0);
    let radius_m = 695700e3;
    let mass_kg = 1.989e30;
    let distance_to_sun_m = 0.0;
    let escape_velocity_mps = 0.0;
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_sun_texture())),
        ..default()
    });
    let mesh = meshes.add(UVSphere {
        radius: radius_m,
        ..default()
    }.into());
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Mass::new(mass_kg),
        Name("Sun".to_string()),
        Sun,
        GridCell::<i64>::default(),
    ));

    // Earth
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let radius_m = 6378137.0;
    let mass_kg = 5.9722e24;
    let distance_to_sun_m = 149.6e9;
    let escape_velocity_mps = 11186.0;
    let mesh = meshes.add(UVSphere {
        radius: radius_m,
        ..default()
    }.into());
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).1).with_scale(scale),
            ..default()
        },
        Mass::new(mass_kg),
        Name("Earth".to_string()),
        Velocity::new(Vec3::new(0.0, escape_velocity_mps, 0.0)),
        floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).0,
    ));

    // Mars
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let radius_m = 3389500.0;
    let mass_kg = 6.4171e23;
    let distance_to_sun_m = 227.9e9;
    let escape_velocity_mps = 5027.0;
    let mesh = meshes.add(UVSphere {
        radius: radius_m,
        ..default()
    }.into());
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).1).with_scale(scale),
            ..default()
        },
        Mass::new(mass_kg),
        Name("Mars".to_string()),
        Velocity::new(Vec3::new(0.0, escape_velocity_mps, 0.0)),
        floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).0,
    ));

    // Mercury
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let radius_m = 2439700.0;
    let mass_kg = 3.3011e23;
    let distance_to_sun_m = 57.9e9;
    let escape_velocity_mps = 4276.0;
    let mesh = meshes.add(UVSphere {
        radius: radius_m,
        ..default()
    }.into());
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).1).with_scale(scale),
            ..default()
        },
        Mass::new(mass_kg),
        Name("Mercury".to_string()),
        Velocity::new(Vec3::new(0.0, escape_velocity_mps, 0.0)),
        floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).0,
    ));

    // Venus
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let radius_m = 6051800.0;
    let mass_kg = 4.8675e24;
    let distance_to_sun_m = 108.2e9;
    let escape_velocity_mps = 10360.0;
    let mesh = meshes.add(UVSphere {
        radius: radius_m,
        ..default()
    }.into());
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).1).with_scale(scale),
            ..default()
        },
        Mass::new(mass_kg),
        Name("Venus".to_string()),
        Velocity::new(Vec3::new(0.0, escape_velocity_mps, 0.0)),
        floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).0,
    ));

    // Jupiter
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let radius_m = 69911000.0;
    let mass_kg = 1.8982e27;
    let distance_to_sun_m = 778.6e9;
    let escape_velocity_mps = 59500.0;
    let mesh = meshes.add(UVSphere {
        radius: radius_m,
        ..default()
    }.into());
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).1).with_scale(scale),
            ..default()
        },
        Mass::new(mass_kg),
        Name("Jupiter".to_string()),
        Velocity::new(Vec3::new(0.0, escape_velocity_mps, 0.0)),
        floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).0,
    ));

    // Saturn
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let radius_m = 58232000.0;
    let mass_kg = 5.6834e26;
    let distance_to_sun_m = 1433.5e9;
    let escape_velocity_mps = 35200.0;
    let mesh = meshes.add(UVSphere {
        radius: radius_m,
        ..default()
    }.into());
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).1).with_scale(scale),
            ..default()
        },
        Mass::new(mass_kg),
        Name("Saturn".to_string()),
        Velocity::new(Vec3::new(0.0, escape_velocity_mps, 0.0)),
        floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).0,
    ));

    // Uranus
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let radius_m = 25362000.0;
    let mass_kg = 8.6810e25;
    let distance_to_sun_m = 2872.5e9;
    let escape_velocity_mps = 21300.0;
    let mesh = meshes.add(UVSphere {
        radius: radius_m,
        ..default()
    }.into());
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).1).with_scale(scale),
            ..default()
        },
        Mass::new(mass_kg),
        Name("Uranus".to_string()),
        Velocity::new(Vec3::new(0.0, escape_velocity_mps, 0.0)),
        floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).0,
    ));

    // Neptune
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let radius_m = 24622000.0;
    let mass_kg = 1.0241e26;
    let distance_to_sun_m = 4495.1e9;
    let escape_velocity_mps = 24000.0;
    let mesh = meshes.add(UVSphere {
        radius: radius_m,
        ..default()
    }.into());
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).1).with_scale(scale),
            ..default()
        },
        Mass::new(mass_kg),
        Name("Neptune".to_string()),
        Velocity::new(Vec3::new(0.0, escape_velocity_mps, 0.0)),
        floating_origin_settings.translation_to_grid::<i64>(DVec3::new(0.0, 0.0, -distance_to_sun_m)).0,
    ));

    // Spawn a random amount of asteroids at a random distance and velocity around the sun.
    let mut rng = rand::thread_rng();
    let num_asteroids = rng.gen_range(10..20);
    for no in 0..num_asteroids {
        let radius_m = rng.gen_range(1000.0..10000.0);
        let mass_kg = rng.gen_range(1.0e10..1.0e20);
        let distance_to_sun_m_x = rng.gen_range(1.0e10..1.0e11) - 1.5e10;
        let distance_to_sun_m_y = rng.gen_range(1.0e10..1.0e11) - 1.5e10;
        let distance_to_sun_m_z = rng.gen_range(1.0e10..1.0e11) - 1.5e10;
        let escape_velocity_mps_x = rng.gen_range(1.0e3..1.0e4) - 1.5e3;
        let escape_velocity_mps_y = rng.gen_range(1.0e3..1.0e4) - 1.5e3;
        let escape_velocity_mps_z = rng.gen_range(1.0e3..1.0e4) - 1.5e3;
        let mesh = meshes.add(UVSphere {
            radius: radius_m,
            ..default()
        }.into());
        commands.spawn((
            PbrBundle {
                mesh,
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(images.add(uv_debug_texture())),
                    ..default()
                }),
                transform: Transform::from_translation(floating_origin_settings.translation_to_grid::<i64>(DVec3::new(distance_to_sun_m_x, distance_to_sun_m_y, distance_to_sun_m_z)).1).with_scale(scale),
                ..default()
            },
            Mass::new(mass_kg),
            MassNoEffect,
            Name(format!("Asteroid {}", no).to_string()),
            Velocity::new(Vec3::new(escape_velocity_mps_x, escape_velocity_mps_y, escape_velocity_mps_z)),
            floating_origin_settings.translation_to_grid::<i64>(DVec3::new(distance_to_sun_m_x, distance_to_sun_m_y, distance_to_sun_m_z)).0,
        ));
    }

}