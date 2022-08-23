use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::action::Action;


#[derive(Deserialize, Serialize)]
pub struct Config {
    pub smarthome: SmarthomeConfig,
    pub hardware: HardwareConfig,
    pub actions: Vec<Action>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            smarthome: SmarthomeConfig::default(),
            hardware: HardwareConfig::default(),
            actions: vec![Action::default(), Action::default()],
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SmarthomeConfig {
    pub url: String,
    pub token: String,
}

impl Default for SmarthomeConfig {
    fn default() -> Self {
        Self {
            url: "http://smarthome.box".to_string(),
            token: "your-token".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct HardwareConfig {
    pub enabled: bool,
    pub pin: u8,
}

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Parse(toml::de::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self::Parse(e)
    }
}

pub fn read_config(file_path: &str) -> Result<Config, Error> {
    // Create or read the file based on it's current existence
    let path = Path::new(file_path);
    match &path.exists() {
        true => {
            // Read the file
            let raw_config = fs::read_to_string(&path)?;
            debug!("Found existing config file at {file_path}");
            Ok::<Config, Error>(toml::from_str::<Config>(&raw_config)?)
        }
        false => {
            // Create the file and it's parent directories
            fs::create_dir_all(&path.parent().unwrap())?;
            let mut file = File::create(path)?;
            file.write_all(include_bytes!("default_config.toml"))?;
            info!("Created new config file at {file_path}");
            Ok(Config::default())
        }
    }
}
