use bevy::core::Name;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::scene::SceneInstance;
use crate::bevy_stupid::debug_print_components_to_console;

#[derive(Default, Component)]
pub struct ProcessGEntity;


struct GEntityMapEntry {
    prefix: String,
    keep_mesh_render: bool,
    hook: Box<dyn Fn(EntityRef, &mut Commands) + Send + Sync + 'static>,
}

#[derive(Default, Resource)]
pub struct GEntityMap {
    map: Vec<GEntityMapEntry>,
}

impl GEntityMap {
    pub fn new() -> Self {
        Self {
            map: vec![]
        }
    }

    pub fn add(&mut self, prefix: String, keep_mesh_render: bool, hook: Box<dyn Fn(EntityRef, &mut Commands) + Send + Sync + 'static>) {
        self.map.push(GEntityMapEntry {
            prefix,
            keep_mesh_render,
            hook,
        });
    }
}


pub fn processs_gentity_gltf_scene(
    unloaded_instances: Query<(Entity, &SceneInstance), With<ProcessGEntity>>,
    scene_manager: Res<SceneSpawner>,
    gentity_map: Res<GEntityMap>,
    world: &World,
    mut cmds: Commands,
) {
    for (entity, instance) in unloaded_instances.iter() {
        if scene_manager.instance_is_ready(**instance) {
            cmds.entity(entity).remove::<ProcessGEntity>();
        }
        let entities = scene_manager
            .iter_instance_entities(**instance)
            .chain(std::iter::once(entity));
        for entity_ref in entities.filter_map(|e| world.get_entity(e)) {
            let name_opt = entity_ref.get::<Name>();
            if let Some(name) = name_opt {
                for entry in gentity_map.map.iter() {
                    if name.len() > entry.prefix.len() && name.starts_with(&entry.prefix) {
                        (entry.hook)(entity_ref, &mut cmds);
                        if !entry.keep_mesh_render {
                            cmds.entity(entity_ref.id())
                                .remove::<Visibility>()
                                .remove::<InheritedVisibility>()
                                .remove::<ViewVisibility>()
                                .remove::<Aabb>()
                                .remove::<Handle<StandardMaterial>>()
                                .remove::<Handle<Mesh>>();
                        }
                    }
                }
            }
        }
    }
}