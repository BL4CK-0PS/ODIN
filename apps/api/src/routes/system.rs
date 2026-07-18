use crate::error::AppError;
use crate::response::ApiResponse;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub investigations: usize,
    pub memories: usize,
    pub entities: usize,
    pub matches: usize,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "OK".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}

pub async fn version() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "name": "ODIN - Operational Defense Intelligence Network",
        "build": env!("CARGO_PKG_VERSION"),
    }))
}

pub async fn stats(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<ApiResponse<StatsResponse>>), AppError> {
    let investigations = match state.incidents.read() {
        Ok(incidents) => incidents.len(),
        Err(_) => 0,
    };

    let memories = state.memory.list_all().unwrap_or_default().len();

    let entities = if let Ok(ents) = state.entities.read() {
        let mut unique_ents = std::collections::HashSet::new();
        for list in ents.values() {
            for e in list {
                unique_ents.insert((e.name.clone(), format!("{:?}", e.entity_type)));
            }
        }
        unique_ents.len()
    } else {
        0
    };

    let mut matches = 0;
    if let Ok(incidents) = state.incidents.read() {
        let candidates: Vec<odin_core::odin_kernel::MemoryObject> =
            state.memory.list_all().unwrap_or_default();
        for query in incidents.values() {
            if let Ok(results) = state.retrieval.search(query, &candidates, 10) {
                for res in results {
                    if res.memory.incident_id != query.id && res.score.overall > 0.7 {
                        matches += 1;
                    }
                }
            }
        }
    }

    Ok(ApiResponse::ok(StatsResponse {
        investigations,
        memories,
        entities,
        matches,
    }))
}
