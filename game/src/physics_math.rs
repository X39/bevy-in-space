/**
 * physics_math.rs
 *
 */


pub mod single {
    /**
     * The gravitational constant in meters cubed per kilogram per second squared.
     * (m^3 * kg^-1 * s^-2)
     */
    pub const GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11;

    /**
     * #### Description
     * Computes the gravitational force between two objects in Newtons.
     *
     * #### Parameters
     * - `mass_a` -- The mass of the first object in kilograms.
     * - `mass_b` -- The mass of the second object in kilograms.
     * - `distance_squared` -- The distance between the two objects in meters squared.
     *
     * #### Returns
     * The gravitational force between the two objects in Newtons.
     */
    pub fn compute_gravitational_force2(mass_a: f32, mass_b: f32, distance_squared: f32) -> f32 {
        GRAVITATIONAL_CONSTANT * mass_a * mass_b / distance_squared
    }

    /**
     * #### Description
     * Computes the gravitational force between two objects in Newtons.
     *
     * #### Parameters
     * - `mass_a` -- The mass of the first object in kilograms.
     * - `mass_b` -- The mass of the second object in kilograms.
     * - `distance` -- The distance between the two objects in meters.
     *
     * #### Returns
     * The gravitational force between the two objects in Newtons.
     */
    pub fn compute_gravitational_force(mass_a: f32, mass_b: f32, distance: f32) -> f32 {
        compute_gravitational_force2(mass_a, mass_b, distance * distance)
    }
}

pub mod double {
    /**
     * The gravitational constant in meters cubed per kilogram per second squared.
     * (m^3 * kg^-1 * s^-2)
     */
    pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;

    /**
     * #### Description
     * Computes the gravitational force between two objects in Newtons.
     *
     * #### Parameters
     * - `mass_a` -- The mass of the first object in kilograms.
     * - `mass_b` -- The mass of the second object in kilograms.
     * - `distance_squared` -- The distance between the two objects in meters squared.
     *
     * #### Returns
     * The gravitational force between the two objects in Newtons.
     */
    pub fn compute_gravitational_force2(mass_a: f64, mass_b: f64, distance_squared: f64) -> f64 {
        GRAVITATIONAL_CONSTANT * mass_a * mass_b / distance_squared
    }

    /**
     * #### Description
     * Computes the gravitational force between two objects in Newtons.
     *
     * #### Parameters
     * - `mass_a` -- The mass of the first object in kilograms.
     * - `mass_b` -- The mass of the second object in kilograms.
     * - `distance` -- The distance between the two objects in meters.
     *
     * #### Returns
     * The gravitational force between the two objects in Newtons.
     */
    pub fn compute_gravitational_force(mass_a: f64, mass_b: f64, distance: f64) -> f64 {
        compute_gravitational_force2(mass_a, mass_b, distance * distance)
    }
}