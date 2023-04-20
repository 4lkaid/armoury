use anyhow::Result;
use tracing_appender::non_blocking::WorkerGuard;

mod config;
mod error;
mod route;
mod service;

pub async fn run() -> Result<WorkerGuard> {
    let worker_guard = config::init().await?;
    Ok(worker_guard)
}
