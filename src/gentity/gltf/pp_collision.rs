use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use crate::gentity::gltf::hook::GEntityMap;

pub fn setup_pp_collision(
    mut gentity_map: ResMut<GEntityMap>
) {
    gentity_map.add("collider.".into(), false, Box::new(|entity, cmds| {
        let transform_opt = entity.get::<Transform>();
        if let Some(transform) = transform_opt {
            let mut cloned_transform = transform.clone();
            cloned_transform.scale = Vec3::ONE;
            cmds
                .entity(entity.id())
                .remove::<Transform>()
                .insert(cloned_transform)
                .insert(RigidBody::Kinematic) // ToDo: Figure out why Position is not updated
                .insert(Collider::cuboid(transform.scale.x as f64, transform.scale.y as f64, transform.scale.z as f64));
        } else {
            warn!("No transform found for entity: {:?}", entity.id());
        }
    }));
}