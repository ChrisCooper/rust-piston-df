use std::env;
use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
pub struct GameSettings {
    map: Map
}

#[derive(Debug, Deserialize)]
struct Map {
    x_width: u32,
    y_length: u32,
    z_height: u32
}

impl GameSettings {
    pub fn new_from_defaults() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("GAME_RUN_MODE").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // You can deserialize (and thus freeze) the entire configuration as
        return s.try_into();
    }
}