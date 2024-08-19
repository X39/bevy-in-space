use bevy::ecs::component::{ComponentId, ComponentInfo};
use bevy::math::{DVec3, Vec3};
use bevy::prelude::{Entity, World};

pub fn vec3_to_dvec3(v: Vec3) -> DVec3 {
    DVec3::new(v.x as f64, v.y as f64, v.z as f64)
}

pub fn dvec3_to_vec3(v: DVec3) -> Vec3 {
    Vec3::new(v.x as f32, v.y as f32, v.z as f32)
}



pub fn debug_print_components_to_console(world: &World, entity: Entity) {
    let names = all_component_infos(&world, entity)
        .map(|info| info.name())
        .collect::<Vec<_>>()
        .join(", ");
    println!("{:?}: {{ {names} }}", entity, names = names);
}
fn all_component_ids<'a>(
    world: &'a World,
    entity: Entity,
) -> impl Iterator<Item = ComponentId> + 'a {
    let components = world.components();
    for archetype in world.archetypes().iter() {
        if archetype.entities().iter().any(|e| e.entity() == entity) {
            return archetype.components();
        }
    }
    world.archetypes().empty().components()
}
fn all_component_infos<'a>(
    world: &'a World,
    entity: Entity,
) -> impl Iterator<Item = &'a ComponentInfo> + 'a {
    let components = world.components();
    all_component_ids(world, entity).map(|id| {
        components
            .get_info(id)
            .expect("Component id without info, this shouldnt happen..")
    })
}
