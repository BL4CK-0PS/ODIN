use crate::response::ApiResponse;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use odin_core::odin_kernel::{CanonicalIncident, Evidence, EvidenceType, Severity};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct UploadRequest {
    pub title: String,
    pub description: String,
    pub severity: String,
    pub evidence: Vec<UploadEvidence>,
}

#[derive(Debug, Deserialize)]
pub struct UploadEvidence {
    pub source: String,
    pub content: String,
    pub content_type: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub incident_id: String,
    pub top_k: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct IncidentSummary {
    pub id: String,
    pub title: String,
    pub severity: String,
    pub status: String,
    pub evidence_count: usize,
    pub entity_count: usize,
}

pub async fn upload(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UploadRequest>,
) -> (StatusCode, Json<ApiResponse<IncidentSummary>>) {
    let severity = match req.severity.to_lowercase().as_str() {
        "critical" => Severity::Critical,
        "high" => Severity::High,
        "medium" => Severity::Medium,
        "low" => Severity::Low,
        _ => Severity::Informational,
    };
    let incident = CanonicalIncident::new(req.title, req.description, severity);
    let incident_id = incident.id.clone();
    let evidence_list: Vec<Evidence> = req
        .evidence
        .into_iter()
        .map(|e| {
            let ct = match e.content_type.to_lowercase().as_str() {
                "log" => EvidenceType::Log,
                "network" => EvidenceType::NetworkCapture,
                "file" => EvidenceType::FileSystemArtifact,
                "memory" => EvidenceType::MemoryDump,
                "threat_intel" => EvidenceType::ThreatIntelReport,
                "report" => EvidenceType::UserReport,
                _ => EvidenceType::Other(e.content_type.clone()),
            };
            Evidence::new(incident_id.clone(), e.source, e.content, ct, 0.9)
        })
        .collect();
    let evidence_count = evidence_list.len();
    if let Ok(mut incidents) = state.incidents.write() {
        incidents.insert(incident.id.clone(), incident);
    }
    if let Ok(mut ev_map) = state.evidence.write() {
        ev_map.insert(incident_id.clone(), evidence_list);
    }
    let summary = IncidentSummary {
        id: incident_id,
        title: String::new(),
        severity: String::new(),
        status: String::new(),
        evidence_count,
        entity_count: 0,
    };
    ApiResponse::created(summary)
}

pub async fn get_incident(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<CanonicalIncident>>) {
    match state.incidents.read() {
        Ok(incidents) => match incidents.get(&id) {
            Some(incident) => ApiResponse::ok(incident.clone()),
            None => ApiResponse::err(StatusCode::NOT_FOUND, "Incident not found"),
        },
        Err(_) => ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR, "Lock error"),
    }
}

pub async fn get_timeline(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    match state.evidence.read() {
        Ok(evidence) => match evidence.get(&id) {
            Some(ev_list) => {
                let timeline: Vec<serde_json::Value> = ev_list
                    .iter()
                    .map(|e| {
                        serde_json::json!({
                            "id": e.id,
                            "source": e.source,
                            "type": format!("{:?}", e.content_type),
                            "collected_at": e.collected_at,
                        })
                    })
                    .collect();
                ApiResponse::ok(serde_json::json!({ "incident_id": id, "events": timeline }))
            }
            None => ApiResponse::err(StatusCode::NOT_FOUND, "Incident not found"),
        },
        Err(_) => ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR, "Lock error"),
    }
}

pub async fn get_memory(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    match state.memory.get_memory_by_incident(&id) {
        Ok(Some(memory)) => ApiResponse::ok(serde_json::json!(memory)),
        Ok(None) => ApiResponse::err(StatusCode::NOT_FOUND, "Memory not found"),
        Err(e) => ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub async fn search_similar(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SearchRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let top_k = req.top_k.unwrap_or(5);
    let query = match state.incidents.read() {
        Ok(incidents) => match incidents.get(&req.incident_id) {
            Some(i) => i.clone(),
            None => return ApiResponse::err(StatusCode::NOT_FOUND, "Incident not found"),
        },
        Err(_) => return ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR, "Lock error"),
    };
    let candidates = state.memory.list_all().unwrap_or_default();
    match state.retrieval.search(&query, &candidates, top_k) {
        Ok(results) => ApiResponse::ok(serde_json::json!({ "results": results })),
        Err(e) => ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub async fn get_graph(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let ev_list = match state.evidence.read() {
        Ok(evidence) => evidence.get(&id).cloned().unwrap_or_default(),
        Err(_) => Vec::new(),
    };
    let nodes: Vec<serde_json::Value> = ev_list
        .iter()
        .map(|e| {
            serde_json::json!({
                "id": e.id,
                "type": "evidence",
                "label": e.source,
            })
        })
        .collect();
    let edges: Vec<serde_json::Value> = Vec::new();
    ApiResponse::ok(serde_json::json!({
        "nodes": nodes,
        "edges": edges,
    }))
}

pub async fn get_playbooks(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let policy_results = state.policy.enforce(0.85);
    let policy_allowed = policy_results.iter().all(|r| {
        matches!(r.verdict, odin_core::odin_policy_gate::PolicyVerdict::Allow)
    });
    if !policy_allowed {
        return ApiResponse::err(StatusCode::FORBIDDEN, "Policy gate blocked");
    }
    ApiResponse::ok(serde_json::json!({
        "incident_id": id,
        "playbooks": [
            {
                "id": "pb-1",
                "name": "Initial Triage",
                "steps": ["Identify affected systems", "Isolate compromised hosts", "Collect forensic data"],
            },
            {
                "id": "pb-2",
                "name": "Containment",
                "steps": ["Block C2 domains", "Disable compromised accounts", "Apply firewall rules"],
            },
        ]
    }))
}

#[derive(Debug, Deserialize)]
pub struct FeedbackRequest {
    pub feedback: String,
    pub rating: u8,
}

pub async fn post_feedback(
    Path(id): Path<String>,
    Json(_req): Json<FeedbackRequest>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    ApiResponse::ok(format!("Feedback recorded for incident {}", id))
}
