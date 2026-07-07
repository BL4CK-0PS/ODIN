use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "OK".into(),
        version: "0.1.0".into(),
    })
}

pub async fn version() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "version": "0.1.0",
        "name": "ODIN - Operational Defense Intelligence Network",
        "build": env!("CARGO_PKG_VERSION"),
    }))
}
