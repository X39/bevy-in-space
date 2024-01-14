use bevy::app::{App, Plugin, PostUpdate, Startup, Update};
use bevy::math::{DQuat, DVec3, Quat};
use bevy::prelude::{Component, Entity, Query, Transform, Without};

/**
 * #### Description
 * A component that describes the orbit of an entity.
 */
#[derive(Debug, Component, Default, Clone, Copy, PartialEq)]
pub struct Orbit {
    /**
     * #### Description
     * The average distance from the center of the ellipse to its edge.
     * This is the average of the periapsis and apoapsis.
     *
     * #### Remarks
     * - The periapsis is the closest point to the center of the ellipse.
     * - The apoapsis is the farthest point from the center of the ellipse.
     * - The semi-major axis is the average of the periapsis and apoapsis.
     * - The semi-major axis is the radius of the ellipse.
     *
     * #### Unit
     * Meters
     */
    pub semi_major_axis_m: f64,

    /**
     * #### Description
     * The eccentricity of the ellipse.
     *
     * #### Remarks
     * - The eccentricity is a measure of how much the ellipse deviates from a circle.
     * - The eccentricity is 0 for a circle.
     * - The eccentricity is 1 for a parabola.
     * - The eccentricity is between 0 and 1 for an ellipse.
     * - The eccentricity is greater than 1 for a hyperbola.
     *
     * #### Unit
     * None
     */
    pub eccentricity: f64,

    /**
     * #### Description
     * The angle between the reference direction and the periapsis.
     *
     * #### Remarks
     * - The reference direction is the direction of the semi-major axis.
     * - The periapsis is the closest point to the center of the ellipse.
     *
     * #### Unit
     * Radians
     */
    pub inclination_rad: f64,

    /**
     * #### Description
     * The angle between the reference direction and the ascending node.
     *
     * #### Remarks
     * - The reference direction is the direction of the semi-major axis.
     * - The ascending node is the point where the orbit crosses the reference plane from below.
     * - The ascending node is the point where the orbit crosses the reference plane from below.
     *
     * #### Unit
     * Radians
     */
    pub longitude_of_ascending_node_rad: f64,

    /**
     * #### Description
     * The angle between the periapsis and the ascending node.
     *
     * #### Remarks
     * - The periapsis is the closest point to the center of the ellipse.
     * - The ascending node is the point where the orbit crosses the reference plane from below.
     *
     * #### Unit
     * Radians
     */
    pub argument_of_periapsis_rad: f64,

    // /**
    //  * #### Description
    //  * The angle between the periapsis and the current position of the orbiting body.
    //  *
    //  * #### Remarks
    //  * - The periapsis is the closest point to the center of the ellipse.
    //  * - The current position of the orbiting body is the current position of the entity.
    //  *
    //  * #### Unit
    //  * Radians
    //  */
    // pub true_anomaly_rad: f64,
}

impl Orbit {

    /**
    * #### Description
    * Calculates the position of the orbiting body at the given time.
    *
    * #### Remarks
    * - The position is relative to the center of the ellipse.
    * - The position is relative to the reference plane.
    *
    * #### Unit
    * Meters
    *
    * #### Parameters
    * - `time`: The time at which to calculate the position in seconds.
    * - `orbited_mass`: The mass of the body being orbited in kilograms.
    */

    pub fn calculate_position_f64(&self, time: f64, orbited_mass: f64) -> (f64, f64, f64) {
        let orbital_period_s = (2.0 * std::f64::consts::PI) * (self.semi_major_axis_m.powi(3) / (6.67430e-11 * orbited_mass)).sqrt();
        let mean_motion_rad = (2.0 * std::f64::consts::PI) / orbital_period_s;
        let mean_anomaly_rad = mean_motion_rad * time;
        let eccentric_anomaly_rad = self.calculate_eccentric_anomaly_rad(mean_anomaly_rad);
        let true_anomaly_rad = self.calculate_true_anomaly_rad(eccentric_anomaly_rad);
        let radius_m = self.calculate_radius_m(true_anomaly_rad);
        let position_x_m = radius_m * true_anomaly_rad.cos();
        let position_y_m = radius_m * true_anomaly_rad.sin();
        (position_x_m, position_y_m, true_anomaly_rad)
    }

    fn calculate_eccentric_anomaly_rad(&self, mean_anomaly_rad: f64) -> f64 {
        let mut eccentric_anomaly_rad = mean_anomaly_rad + self.eccentricity * mean_anomaly_rad.cos();
        let mut delta_eccentric_anomaly_rad = eccentric_anomaly_rad - self.eccentricity * eccentric_anomaly_rad.sin() - mean_anomaly_rad;
        while delta_eccentric_anomaly_rad.abs() > 1.0e-6 {
            eccentric_anomaly_rad -= delta_eccentric_anomaly_rad / (1.0 - self.eccentricity * eccentric_anomaly_rad.cos());
            delta_eccentric_anomaly_rad = eccentric_anomaly_rad - self.eccentricity * eccentric_anomaly_rad.sin() - mean_anomaly_rad;
        }
        eccentric_anomaly_rad
    }

    fn calculate_true_anomaly_rad(&self, eccentric_anomaly_rad: f64) -> f64 {
        let true_anomaly_rad = 2.0 * eccentric_anomaly_rad.atan2((1.0 + self.eccentricity).sqrt() * (1.0 - self.eccentricity).sqrt());
        true_anomaly_rad
    }

    fn calculate_radius_m(&self, true_anomaly_rad: f64) -> f64 {
        let radius_m = self.semi_major_axis_m * (1.0 - self.eccentricity.powi(2)) / (1.0 + self.eccentricity * true_anomaly_rad.cos());
        radius_m
    }

}
