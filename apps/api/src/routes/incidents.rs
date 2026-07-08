use crate::response::ApiResponse;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use odin_core::odin_kernel::{CanonicalIncident, Evidence, EvidenceType, Severity, MemoryObject, Entity, EntityType};
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

#[derive(Debug, Deserialize)]
pub struct TextSearchRequest {
    pub query: String,
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

fn extract_mitre_techniques(text: &str) -> Vec<String> {
    let mut techniques = Vec::new();
    for word in text.split(|c: char| !c.is_alphanumeric()) {
        if word.len() == 5 && word.starts_with('T') {
            if word.chars().skip(1).all(|c| c.is_ascii_digit()) {
                techniques.push(word.to_string());
            }
        }
    }
    techniques
}

fn extract_entities_from_content(content: &str) -> Vec<(EntityType, String)> {
    let mut entities = Vec::new();
    
    // Split by common delimiters
    let words: Vec<&str> = content.split(|c: char| c.is_whitespace() || c == ',' || c == ';' || c == '=' || c == '"' || c == '\'').collect();
    
    for word in words {
        let clean = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '.' && c != '-' && c != '_');
        if clean.is_empty() {
            continue;
        }
        
        // Match IP Address
        if clean.contains('.') {
            let parts: Vec<&str> = clean.split('.').collect();
            if parts.len() == 4 && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()) && !p.is_empty()) {
                if parts.iter().all(|p| p.parse::<u8>().is_ok()) {
                    entities.push((EntityType::IpAddress, clean.to_string()));
                    continue;
                }
            }
        }
        
        // Match Files & Processes
        if clean.ends_with(".exe") || clean.ends_with(".dll") || clean.ends_with(".ps1") || clean.ends_with(".bat") {
            if clean == "powershell.exe" || clean == "cmd.exe" || clean == "rundll32.exe" || clean == "svchost.exe" {
                entities.push((EntityType::Process, clean.to_string()));
            } else {
                entities.push((EntityType::File, clean.to_string()));
            }
            continue;
        }
        
        // Match Domains
        if clean.contains('.') && !clean.starts_with('.') && !clean.ends_with('.') {
            let suffix = clean.split('.').last().unwrap_or("");
            if suffix == "com" || suffix == "net" || suffix == "org" || suffix == "xyz" || suffix == "local" {
                entities.push((EntityType::Domain, clean.to_string()));
                continue;
            }
        }
        
        // Match standard process names
        if clean == "powershell" || clean == "cmd" || clean == "rundll32" {
            entities.push((EntityType::Process, clean.to_string()));
            continue;
        }
    }
    
    // Add default user if mentioned
    if content.to_lowercase().contains("admin") {
        entities.push((EntityType::User, "Administrator".to_string()));
    }
    if content.to_lowercase().contains("system") {
        entities.push((EntityType::User, "SYSTEM".to_string()));
    }
    
    entities
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
    
    let mut incident = CanonicalIncident::new(req.title.clone(), req.description.clone(), severity);
    let incident_id = incident.id.clone();
    
    let mut evidence_list = Vec::new();
    let mut ext_techniques = Vec::new();
    let mut ext_entities = Vec::new();
    
    for e in req.evidence {
        let ct = match e.content_type.to_lowercase().as_str() {
            "log" => EvidenceType::Log,
            "network" => EvidenceType::NetworkCapture,
            "file" => EvidenceType::FileSystemArtifact,
            "memory" => EvidenceType::MemoryDump,
            "threat_intel" => EvidenceType::ThreatIntelReport,
            "report" => EvidenceType::UserReport,
            _ => EvidenceType::Other(e.content_type.clone()),
        };
        
        let tecs = extract_mitre_techniques(&e.content);
        ext_techniques.extend(tecs);
        
        let ents = extract_entities_from_content(&e.content);
        ext_entities.extend(ents);
        
        evidence_list.push(Evidence::new(incident_id.clone(), e.source, e.content, ct, 0.9));
    }
    
    // De-duplicate techniques
    ext_techniques.sort();
    ext_techniques.dedup();
    incident.mitre_techniques = ext_techniques;
    
    // De-duplicate entities
    ext_entities.sort_by_key(|e| (format!("{:?}", e.0), e.1.clone()));
    ext_entities.dedup_by(|a, b| format!("{:?}", a.0) == format!("{:?}", b.0) && a.1 == b.1);
    
    let mut entities = Vec::new();
    for (etype, ename) in ext_entities {
        let ent = Entity::new(ename, etype, serde_json::json!({}));
        entities.push(ent);
    }
    
    incident.entity_ids = entities.iter().map(|e| e.id.clone()).collect();
    incident.evidence_ids = evidence_list.iter().map(|e| e.id.clone()).collect();
    
    let severity_str = match incident.severity {
        Severity::Critical => "Critical",
        Severity::High => "High",
        Severity::Medium => "Medium",
        Severity::Low => "Low",
        Severity::Informational => "Informational",
    };
    incident.tags = vec![severity_str.to_lowercase()];
    if !incident.mitre_techniques.is_empty() {
        incident.tags.push("mitre-mapped".to_string());
    }

    // Run Intelligence Engine pipeline
    if let Ok(mut intel) = state.intelligence.write() {
        let _ = intel.analyze(&evidence_list);
    }
    
    // Store in memory engine
    let _ = state.memory.store_incident(&incident);
    
    let evidence_count = evidence_list.len();
    let entity_count = entities.len();
    
    tracing::info!("UPLOAD: inserting incident {} (evidence_count={}, entity_count={})", incident_id, evidence_count, entity_count);
    
    if let Ok(mut incidents) = state.incidents.write() {
        incidents.insert(incident_id.clone(), incident);
    }
    if let Ok(mut ev_map) = state.evidence.write() {
        ev_map.insert(incident_id.clone(), evidence_list);
    }
    if let Ok(mut ent_map) = state.entities.write() {
        ent_map.insert(incident_id.clone(), entities);
    }
    
    let summary = IncidentSummary {
        id: incident_id,
        title: req.title,
        severity: severity_str.to_string(),
        status: "New".to_string(),
        evidence_count,
        entity_count,
    };
    ApiResponse::created(summary)
}

pub async fn list_incidents(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<ApiResponse<Vec<CanonicalIncident>>>) {
    match state.incidents.read() {
        Ok(incidents) => {
            let list: Vec<CanonicalIncident> = incidents.values().cloned().collect();
            ApiResponse::ok(list)
        }
        Err(_) => ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR, "Lock error"),
    }
}

pub async fn get_incident(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<CanonicalIncident>>) {
    match state.incidents.read() {
        Ok(incidents) => {
            match incidents.get(&id) {
                Some(incident) => ApiResponse::ok(incident.clone()),
                None => ApiResponse::err(StatusCode::NOT_FOUND, "Incident not found"),
            }
        }
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
                            "content": e.content,
                            "trust_score": e.trust_score,
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

pub async fn list_memories(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<ApiResponse<Vec<MemoryObject>>>) {
    match state.memory.list_all() {
        Ok(memories) => ApiResponse::ok(memories),
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

pub async fn search_text(
    State(state): State<Arc<AppState>>,
    Json(req): Json<TextSearchRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let top_k = req.top_k.unwrap_or(5);
    
    // Create a dummy incident to search with
    let mut dummy_query = CanonicalIncident::new(
        "Search Query".into(),
        req.query.clone(),
        Severity::Informational,
    );
    
    let ext_techniques = extract_mitre_techniques(&req.query);
    dummy_query.mitre_techniques = ext_techniques;
    
    let keywords = vec!["ransomware", "phishing", "credentials", "dll", "powershell", "persistence"];
    let mut tags = Vec::new();
    let query_lower = req.query.to_lowercase();
    for kw in keywords {
        if query_lower.contains(kw) {
            tags.push(kw.to_string());
        }
    }
    dummy_query.tags = tags;
    
    let candidates = state.memory.list_all().unwrap_or_default();
    match state.retrieval.search(&dummy_query, &candidates, top_k) {
        Ok(results) => ApiResponse::ok(serde_json::json!({ "results": results })),
        Err(e) => ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub async fn get_graph(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    
    // Add incident node
    if let Ok(incidents) = state.incidents.read() {
        if let Some(inc) = incidents.get(&id) {
            nodes.push(serde_json::json!({
                "id": inc.id,
                "type": "incident",
                "label": inc.title,
            }));
        }
    }
    
    // Add evidence nodes
    let ev_list = match state.evidence.read() {
        Ok(evidence) => evidence.get(&id).cloned().unwrap_or_default(),
        Err(_) => Vec::new(),
    };
    for e in &ev_list {
        nodes.push(serde_json::json!({
            "id": e.id,
            "type": "evidence",
            "label": format!("{:?}: {}", e.content_type, e.source),
        }));
        // Connect evidence to incident
        edges.push(serde_json::json!({
            "source": e.id,
            "target": id.clone(),
            "type": "evidence_of",
        }));
    }
    
    // Add entity nodes
    let ent_list = match state.entities.read() {
        Ok(entities) => entities.get(&id).cloned().unwrap_or_default(),
        Err(_) => Vec::new(),
    };
    for ent in &ent_list {
        nodes.push(serde_json::json!({
            "id": ent.id,
            "type": format!("{:?}", ent.entity_type).to_lowercase(),
            "label": ent.name,
        }));
        // Connect entity to incident
        edges.push(serde_json::json!({
            "source": ent.id,
            "target": id.clone(),
            "type": "associated_with",
        }));
        // Connect entity to corresponding evidence if matching words are found
        for e in &ev_list {
            if e.content.contains(&ent.name) {
                edges.push(serde_json::json!({
                    "source": ent.id,
                    "target": e.id,
                    "type": "observed_in",
                }));
            }
        }
    }
    
    ApiResponse::ok(serde_json::json!({
        "nodes": nodes,
        "edges": edges,
    }))
}

pub async fn get_global_graph(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    if let Ok(incidents) = state.incidents.read() {
        for inc in incidents.values() {
            nodes.push(serde_json::json!({
                "id": inc.id, "type": "incident", "label": inc.title,
            }));
        }
    }

    if let Ok(evidence) = state.evidence.read() {
        for (inc_id, ev_list) in evidence.iter() {
            for e in ev_list {
                nodes.push(serde_json::json!({
                    "id": e.id, "type": "evidence",
                    "label": format!("{:?}: {}", e.content_type, e.source),
                }));
                edges.push(serde_json::json!({
                    "source": e.id, "target": inc_id, "type": "evidence_of",
                }));
            }
        }
    }

    if let Ok(entities) = state.entities.read() {
        for (inc_id, ent_list) in entities.iter() {
            for ent in ent_list {
                nodes.push(serde_json::json!({
                    "id": ent.id,
                    "type": format!("{:?}", ent.entity_type).to_lowercase(),
                    "label": ent.name,
                }));
                edges.push(serde_json::json!({
                    "source": ent.id, "target": inc_id, "type": "associated_with",
                }));
            }
        }
    }

    ApiResponse::ok(serde_json::json!({ "nodes": nodes, "edges": edges }))
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
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<FeedbackRequest>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    let entry = crate::state::FeedbackEntry {
        feedback: req.feedback,
        rating: req.rating,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs().to_string())
            .unwrap_or_else(|_| "unknown".to_string()),
    };
    match state.feedback.write() {
        Ok(mut feedback_map) => {
            feedback_map.entry(id.clone()).or_default().push(entry);
            ApiResponse::ok(format!("Feedback recorded for incident {}", id))
        }
        Err(_) => ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR, "Lock error"),
    }
}
