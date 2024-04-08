use super::*;

/*
The min distance step size in the buffer can be quite large while keeping movement smooth if
the method of querying the current state of objects interpolates between the next point if one isn't available
based on time and physical distance.

Maybe even artificially bend the interpolated path when zoomed too far using velocity calculated from the previous point .
*/

#[derive(Resource, Default)]
pub struct PhysicsFuture {

}