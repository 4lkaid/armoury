use anyhow::Result;
use serde::Deserialize;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::time::LocalTime;

#[derive(Deserialize)]
pub struct Logger {
    pub directory: String,
    pub file_name_prefix: String,
}

pub fn init(config: &Logger) -> Result<WorkerGuard> {
    let timer = LocalTime::new(time::format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second]",
    )?);

    let file_appender =
        tracing_appender::rolling::daily(&config.directory, &config.file_name_prefix);
    let (non_blocking, worker_guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        // .pretty()
        .with_ansi(false)
        .with_max_level(tracing::Level::DEBUG)
        .with_timer(timer)
        .with_writer(non_blocking)
        .init();
    Ok(worker_guard)
}
