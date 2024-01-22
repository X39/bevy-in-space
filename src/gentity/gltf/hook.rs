use bevy::core::Name;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::scene::SceneInstance;
use crate::spaceship::{all_component_infos, Spaceship};

#[derive(Default, Component)]
pub struct ProcessGEntity;


#[derive(Default, Resource)]
pub struct GEntityMap {
    map: Vec<(String, Box<dyn Fn(&mut Commands) + Send + Sync + 'static>)>,
}

impl GEntityMap {
    pub fn new() -> Self {
        Self {
            map: vec![]
        }
    }

    pub fn add(&mut self, prefix: String, hook: Box<dyn Fn(&mut Commands) + Send + Sync + 'static>) {
        self.map.push((prefix, hook));
    }
}


pub fn setup_gltf_scene(
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
                for (prefix, hook) in gentity_map.map.iter() {
                    if name.len() > prefix.len() && name.starts_with(prefix) {
                        (hook)(&mut cmds);
                    }
                }
            }
        }
    }
}