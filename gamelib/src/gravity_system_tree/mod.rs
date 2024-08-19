use std::collections::VecDeque;
use bevy::{color::palettes::css::{LIGHT_GRAY, WHITE}, math::DVec2, prelude::{Commands, Entity}};
use dynamic_body::DynamicBody;
use itertools::{multizip, Itertools};
use position_generator::PositionGenerator;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator};
use static_body::{StaticBody, StaticPosition};

use crate::visual_object::{VisualObjectBundle, VisualObjectData};


pub mod static_body;
pub mod dynamic_body;
pub mod position_generator;
pub mod builder;
pub mod system_manager;
pub mod massive_object;
pub mod generate;
pub mod system_tree;

/*
Use a binary search pattern to calculate when a body enters a lower system. parent has time scale 100, child 10
If the body is outside the system at time 0, and inside at time 100, check time 90. If outside at 90 use 100, else check 50.
If the body is outside at time 0 and 50, check 80.
If inside at 80, check 70. If outside at 70, use 80. Always choose the first timestep where the body is inside the system according to the child time scale.
This problem would be very hard to solve analytically since the position of the system changes with each time step.
*/


/*
After performing a gravity calculation, all changes should be reported and stored in a future map.
Previous systems can read from the future map like previously to draw objects onto the screen.
This means dynamic bodies need some kind of hashable identifier.


Also factoring in static bodies and sibling systems when calculating gravity for a system's
dynamic bodies could be a good idea if it doesnt impact performance too much.

Using relative coordinates for dynamic bodies causes small accuracy problems.
It really only works well when bodies become trapped in the gravity well of the system, or enter then exit before the velocity of the child system changes too much 

Consider if a body has a velocity of (1,0) when it enters a child system travelling at (0, 1)
The body will get a relative velocity of (1,-1) to offset the velocity of the system.
On the next time step, the body is still in the system, but now the system's velocity is slightly different from (0, 1), more like (-.001, 0.99)
*/

/*
Gravitational acceleration will be updated based on the time step of individual bodies and stored in a map.
Every dynamic body will be updated every individual time step.

For a gravity acceleration update step, it is O(n*k) where n is dynamic bodies and k is static bodies within the system.
For a regular update it is an O(n) operation where n is the total number of dynamic bodies.

This way bodies in high level systems are able to accelerate themselves at any discrete time step instead of only at time step intervals.

Large time jumps are still possible of the dynamic body does not accelerate too much.
A body in a 1000 time scale system can easily calculate its position at time 1000 as long as it doesn't accelerate itself.
If that body begins accelerating itself at time 500, then you can calculate position at time 500, accelerate it, then use the new acceleration to calculate time at 1000.

Possibly just forget fast forwarding so it is super easy to determine when a body leaves a system or enters a lower one. This also removes the need for a wait list.
This might also make it easier to flatten the structure so it can conceptually be reasoned about as a tree but is not actually a tree.

This will certainly slow things down since each body needs to be mutated and checked against systems at every time step, but the potentially expensive gravity calculation
still only occurs according to the time_step of the system.
    (except when a body exits a system or enters a child one, the gravity calculation must be done even if its between time steps. 
    Or does it, this would really only matter for super fast objects that travel a considerable distance relative to the system size within a time_step)
*/


