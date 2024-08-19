/**
 * Contains commonly used math functions.
 */


/**
 * #### Description
 * Returns the distance between two 2D points squared.
 *
 * #### Parameters
 * - `x1` -- The x coordinate of the first point.
 * - `y1` -- The y coordinate of the first point.
 * - `x2` -- The x coordinate of the second point.
 * - `y2` -- The y coordinate of the second point.
 *
 * #### Returns
 * The distance between the two points squared.
 */
pub fn distance2_f64_squared(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let x = x2 - x1;
    let y = y2 - y1;
    x * x + y * y
}

/**
 * #### Description
 * Returns the distance between two 2D points.
 *
 * #### Parameters
 * - `x1` -- The x coordinate of the first point.
 * - `y1` -- The y coordinate of the first point.
 * - `x2` -- The x coordinate of the second point.
 * - `y2` -- The y coordinate of the second point.
 *
 * #### Returns
 * The distance between the two points.
 */
pub fn distance2_f64(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    distance2_f64_squared(x1, y1, x2, y2).sqrt()
}

/**
* #### Description
* Returns the distance between two 3D points squared.
   *
   * #### Parameters
   * - `x1` -- The x coordinate of the first point.
   * - `y1` -- The y coordinate of the first point.
   * - `z1` -- The z coordinate of the first point.
   * - `x2` -- The x coordinate of the second point.
   * - `y2` -- The y coordinate of the second point.
   * - `z2` -- The z coordinate of the second point.
   *
   * #### Returns
   * The distance between the two points squared.
 */
pub fn distance3_f64_squared(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    let x = x2 - x1;
    let y = y2 - y1;
    let z = z2 - z1;
    x * x + y * y + z * z
}

/**
 * #### Description
 * Returns the distance between two 3D points.
 *
 * #### Parameters
 * - `x1` -- The x coordinate of the first point.
 * - `y1` -- The y coordinate of the first point.
 * - `z1` -- The z coordinate of the first point.
 * - `x2` -- The x coordinate of the second point.
 * - `y2` -- The y coordinate of the second point.
 * - `z2` -- The z coordinate of the second point.
 *
 * #### Returns
 * The distance between the two points.
 */
pub fn distance3_f64(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    distance3_f64_squared(x1, y1, z1, x2, y2, z2).sqrt()
}