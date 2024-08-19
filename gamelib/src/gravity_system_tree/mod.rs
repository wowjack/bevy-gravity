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


/*
Updating position every iteration but updating acceleration only based on system time_step causes bodies to slowly increase their orbit.

Possibly rotate the acceleration vector based on the movement of the body. How to do this while always keeping the future path the same.
*/


