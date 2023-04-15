use anyhow::Result;
use serde::Deserialize;
use std::{net::SocketAddr, str::FromStr};

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
        .await
        .unwrap();
    Ok(())
}
