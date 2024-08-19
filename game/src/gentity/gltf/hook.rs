use bevy::core::Name;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::scene::SceneInstance;
use bevy_xpbd_3d::prelude::RigidBody;
use crate::bevy_stupid::debug_print_components_to_console;

#[derive(Default, Component)]
pub struct ProcessGEntity;


struct GEntityMapEntry {
    prefix: String,
    keep_mesh_render: bool,
    hook: Box<dyn Fn(EntityRef, EntityRef, &mut Commands) + Send + Sync + 'static>,
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

    pub fn add(&mut self, prefix: String, keep_mesh_render: bool, hook: Box<dyn Fn(EntityRef, EntityRef, &mut Commands) + Send + Sync + 'static>) {
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
    for (parent_entity, instance) in unloaded_instances.iter() {
        if scene_manager.instance_is_ready(**instance) {
            cmds.entity(parent_entity).remove::<ProcessGEntity>();
        }
        let parent_entity_ref_opt = world.get_entity(parent_entity);
        if parent_entity_ref_opt.is_none() {
            warn!("Parent entity not found: {:?}", parent_entity);
            continue;
        }
        let parent_entity_ref = parent_entity_ref_opt.unwrap();
        cmds.entity(parent_entity).try_insert(RigidBody::Kinematic);
        let entities = scene_manager
            .iter_instance_entities(**instance)
            .chain(std::iter::once(parent_entity));
        for entity_ref in entities.filter_map(|e| world.get_entity(e)) {
            let name_opt = entity_ref.get::<Name>();
            if let Some(name) = name_opt {
                for entry in gentity_map.map.iter() {
                    if name.len() > entry.prefix.len() && name.starts_with(&entry.prefix) {
                        (entry.hook)(parent_entity_ref, entity_ref, &mut cmds);
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