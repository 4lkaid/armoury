use anyhow::Result;
use tracing_appender::non_blocking::WorkerGuard;

mod config;
mod error;
mod route;
mod service;

pub async fn init() -> Result<WorkerGuard> {
    let config = config::Config::load()?;
    let worker_guard = config::logger::init(&config.logger)?;
    config::general::init(&config.general).await?;
    Ok(worker_guard)
}
