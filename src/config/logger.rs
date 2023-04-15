use anyhow::Result;
use serde::Deserialize;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{time::LocalTime, writer::MakeWriterExt},
    prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt,
};

#[derive(Deserialize)]
pub struct Logger {
    pub debug: bool,
    pub directory: String,
    pub file_name_prefix: String,
}

pub fn init(config: &Logger) -> Result<WorkerGuard> {
    let timer = LocalTime::new(time::format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second]",
    )?);

    let debug_layer = if config.debug {
        let debug_layer = tracing_subscriber::fmt::layer()
            .pretty()
            .with_ansi(true)
            .with_timer(timer.clone())
            .with_writer(
                std::io::stdout
                    .with_min_level(tracing::Level::DEBUG)
                    .with_max_level(tracing::Level::DEBUG),
            );
        Some(debug_layer)
    } else {
        None
    };

    let file_appender =
        tracing_appender::rolling::daily(&config.directory, &config.file_name_prefix);
    let (non_blocking, worker_guard) = tracing_appender::non_blocking(file_appender);
    let info_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_timer(timer)
        .with_writer(non_blocking.with_max_level(tracing::Level::INFO));

    tracing_subscriber::registry()
        .with(debug_layer)
        .with(info_layer)
        .init();

    Ok(worker_guard)
}
