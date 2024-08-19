use bevy::prelude::{Component, Entity, Query, Transform, Without};

/**
 * #### Description
 * A component that describes the orbit of an entity.
 */
#[derive(Debug, Component, Default, Clone, Copy, PartialEq)]
pub struct  Orbit {

}
// ToDo: https://orbital-mechanics.space/orbital-maneuvers/impulsive-maneuvers.html