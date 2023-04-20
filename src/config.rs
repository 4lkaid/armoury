use crate::error::ArmouryError;
use anyhow::Result;
use serde::Deserialize;
use tracing_appender::non_blocking::WorkerGuard;

mod general;
mod logger;

#[derive(Deserialize)]
struct Config {
    general: general::General,
    logger: logger::Logger,
}

impl Config {
    fn load() -> Result<Self, ArmouryError> {
        let config = std::fs::read_to_string("armoury.toml")?;
        let config: Config = toml::from_str(&config)?;
        Ok(config)
    }
}

pub async fn init() -> Result<WorkerGuard> {
    let config = Config::load()?;
    let worker_guard = logger::init(&config.logger)?;
    general::init(&config.general).await?;
    Ok(worker_guard)
}
