use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use crate::gentity::gltf::hook::GEntityMap;

pub fn setup_pp_trigger(
    mut gentity_map: ResMut<GEntityMap>
) {
    gentity_map.add("trigger.".into(), true, Box::new(|entity, cmds| {
        let transform_opt = entity.get::<Transform>();
        if let Some(transform) = transform_opt {
            cmds
                .entity(entity.id())
                .insert(Collider::cuboid(transform.scale.x as f64, transform.scale.y as f64, transform.scale.z as f64))
                .insert(Sensor);
        } else {
            warn!("No transform found for entity: {:?}", entity.id());
        }
    }));
}

pub fn print_collisions(
    mut collision_event_reader: EventReader<Collision>,
    mut collision_started_event_reader: EventReader<CollisionStarted>,
    mut collision_ended_event_reader: EventReader<CollisionEnded>,
) {
    for Collision(contacts) in collision_event_reader.read() {
        println!(
            "Entities {:?} and {:?} are colliding",
            contacts.entity1,
            contacts.entity2,
        );
    }

    for CollisionStarted(entity1, entity2) in collision_started_event_reader.read() {
        println!(
            "Entities {:?} and {:?} started colliding",
            entity1,
            entity2,
        );
    }

    for CollisionEnded(entity1, entity2) in collision_ended_event_reader.read() {
        println!(
            "Entities {:?} and {:?} ended colliding",
            entity1,
            entity2,
        );
    }
}