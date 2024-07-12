use std::fs;
use bincode::deserialize;

pub fn load_from_file() -> Result<(), ()> {
    let Ok(data) = fs::read("./save.dat") else { return Err(()) };

    let Ok(data) = deserialize::<Vec<u8>>(&data[..]) else { return Err(()) };

    Err(())
}

pub fn save_to_file(data: Vec<u8>) -> Result<(), ()> {
    todo!();
}