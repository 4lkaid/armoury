use anyhow::Result;
use serde::Deserialize;
use std::{net::SocketAddr, str::FromStr};
use tokio::signal;

use crate::route;

#[derive(Deserialize)]
pub struct General {
    pub listen: String,
}

pub async fn init(config: &General) -> Result<()> {
    let addr = &config.listen;
    let addr = SocketAddr::from_str(addr)?;
    tracing::debug!("正在监听 {}", addr);
    let api = route::api::init();
    axum::Server::bind(&addr)
        .serve(api.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::error!("强制终止服务");
}
