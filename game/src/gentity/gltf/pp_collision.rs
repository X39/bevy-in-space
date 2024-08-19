use bevy::prelude::*;
use bevy_xpbd_3d::parry::shape::SharedShape;
use bevy_xpbd_3d::prelude::*;
use crate::gentity::gltf::hook::GEntityMap;

#[derive(Default, Component)]
pub struct JointEntities(Vec<Entity>);
pub fn setup_pp_collision(
    mut gentity_map: ResMut<GEntityMap>
) {
    gentity_map.add("collider.".into(), false, Box::new(|parent, entity, cmds| {
        let transform_opt = entity.get::<Transform>();
        if let Some(transform) = transform_opt {
            let mut cloned_transform = transform.clone();
            cloned_transform.scale = Vec3::ONE;
            let joint_child = cmds.spawn(FixedJoint::new(entity.id(), parent.id())).id();
            cmds
                .entity(entity.id())
                .remove::<Transform>()
                .insert(cloned_transform)
                .insert(RigidBody::Dynamic) // ToDo: Figure out why Position is not updated
                .insert(SharedShape)
                .insert(Collider::cuboid(transform.scale.x as f64, transform.scale.y as f64, transform.scale.z as f64))
                .insert(JointEntities(vec![joint_child]))
            ;
        } else {
            warn!("No transform found for entity: {:?}", entity.id());
        }
    }));
}