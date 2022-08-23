use std::{io, fs::{File, self}};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    smarthome: SmarthomeConfig,
    hardware: HardwareConfig,
    actions: Vec<Action>,
}

#[derive(Deserialize, Serialize)]
pub struct SmarthomeConfig {
    url: String,
    token: String,
}

#[derive(Deserialize, Serialize)]
pub struct HardwareConfig {
    enabled: bool,
    pin: u8,
}

#[derive(Deserialize, Serialize)]
pub struct Action {
    name: String,
    code: u64,
    homescript: String,
}

pub fn read_config(path: &str) -> Result<Config, io::Error> {
    let raw_config = fs::read_to_string(path)?;

    Ok(todo!())
}
