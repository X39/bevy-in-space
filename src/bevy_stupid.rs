use bevy::math::{DVec3, Vec3};

pub fn vec3_to_dvec3(v: Vec3) -> DVec3 {
    DVec3::new(v.x as f64, v.y as f64, v.z as f64)
}

pub fn dvec3_to_vec3(v: DVec3) -> Vec3 {
    Vec3::new(v.x as f32, v.y as f32, v.z as f32)
}