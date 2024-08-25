use std::{fs, ops::Range};
use bevy::{color::Color, math::DVec2};
use bincode::deserialize;
use rand::{rngs::ThreadRng, Rng};

use crate::{visual_object::VisualObjectData};

pub fn load_from_file() -> Result<(), ()> {
    let Ok(data) = fs::read("./save.dat") else { return Err(()) };

    let Ok(data) = deserialize::<Vec<u8>>(&data[..]) else { return Err(()) };

    Err(())
}

pub fn save_to_file(data: Vec<u8>) -> Result<(), ()> {
    todo!();
}
