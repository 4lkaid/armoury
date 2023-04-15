use anyhow::Result;
use serde::Deserialize;

use crate::error::ArmouryError;

pub mod general;
pub mod logger;

#[derive(Deserialize)]
pub struct Config {
    pub general: general::General,
    pub logger: logger::Logger,
}

impl Config {
    pub fn load() -> Result<Self, ArmouryError> {
        let config = std::fs::read_to_string("armoury.toml")?;
        let config: Config = toml::from_str(&config)?;
        Ok(config)
    }
}
