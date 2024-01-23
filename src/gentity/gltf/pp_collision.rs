use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use crate::gentity::gltf::hook::GEntityMap;

pub fn setup_pp_collision(
    mut gentity_map: ResMut<GEntityMap>
) {
    gentity_map.add("collider.".into(), true, Box::new(|entity, cmds| {
        let transform_opt = entity.get::<Transform>();
        if let Some(transform) = transform_opt {
            cmds
                .entity(entity.id())
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(transform.scale.x as f64, transform.scale.y as f64, transform.scale.z as f64));
        } else {
            warn!("No transform found for entity: {:?}", entity.id());
        }
    }));
}