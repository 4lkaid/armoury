use axum::{routing::get, Router};

use crate::service::health;

pub fn init() -> Router {
    Router::new().route("/health", get(health::live))
}
