use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn live() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "OK"})))
}
