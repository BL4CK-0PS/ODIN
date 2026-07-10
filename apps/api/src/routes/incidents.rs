use crate::error::AppError;
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

fn validate_upload(req: &UploadRequest) -> Result<(), AppError> {
    if req.title.trim().is_empty() {
        return Err(AppError::BadRequest("Title is required".into()));
    }
    if req.title.len() > 500 {
        return Err(AppError::BadRequest("Title must be 500 characters or fewer".into()));
    }
    if req.description.len() > 100_000 {
        return Err(AppError::BadRequest("Description must be 100,000 characters or fewer".into()));
    }
    if req.evidence.is_empty() {
        return Err(AppError::BadRequest("At least one evidence item is required".into()));
    }
    if req.evidence.len() > 1000 {
        return Err(AppError::BadRequest("Maximum 1000 evidence items per incident".into()));
    }
    for (i, e) in req.evidence.iter().enumerate() {
        if e.source.trim().is_empty() {
            return Err(AppError::BadRequest(format!("Evidence {}: source is required", i + 1)));
        }
        if e.source.len() > 500 {
            return Err(AppError::BadRequest(format!("Evidence {}: source must be 500 characters or fewer", i + 1)));
        }
        if e.content.len() > 500_000 {
            return Err(AppError::BadRequest(format!("Evidence {}: content must be 500,000 characters or fewer", i + 1)));
        }
    }
    Ok(())
}

fn parse_evidence_type(s: &str) -> EvidenceType {
    match s.to_lowercase().as_str() {
        "log" => EvidenceType::Log,
        "network" => EvidenceType::NetworkCapture,
        "file" => EvidenceType::FileSystemArtifact,
        "memory" => EvidenceType::MemoryDump,
        "threat_intel" => EvidenceType::ThreatIntelReport,
        "report" => EvidenceType::UserReport,
        _ => EvidenceType::Other(s.to_string()),
    }
}

fn format_severity(s: &Severity) -> &'static str {
    match s {
        Severity::Critical => "Critical",
        Severity::High => "High",
        Severity::Medium => "Medium",
        Severity::Low => "Low",
        Severity::Informational => "Informational",
    }
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
    let words: Vec<&str> = content.split(|c: char| c.is_whitespace() || c == ',' || c == ';' || c == '=' || c == '"' || c == '\'').collect();

    for word in words {
        let clean = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '.' && c != '-' && c != '_');
        if clean.is_empty() {
            continue;
        }

        if clean.contains('.') {
            let parts: Vec<&str> = clean.split('.').collect();
            if parts.len() == 4 && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()) && !p.is_empty()) {
                if parts.iter().all(|p| p.parse::<u8>().is_ok()) {
                    entities.push((EntityType::IpAddress, clean.to_string()));
                    continue;
                }
            }
        }

        if clean.ends_with(".exe") || clean.ends_with(".dll") || clean.ends_with(".ps1") || clean.ends_with(".bat") {
            if clean == "powershell.exe" || clean == "cmd.exe" || clean == "rundll32.exe" || clean == "svchost.exe" {
                entities.push((EntityType::Process, clean.to_string()));
            } else {
                entities.push((EntityType::File, clean.to_string()));
            }
            continue;
        }

        if clean.contains('.') && !clean.starts_with('.') && !clean.ends_with('.') {
            let suffix = clean.split('.').last().unwrap_or("");
            if suffix == "com" || suffix == "net" || suffix == "org" || suffix == "xyz" || suffix == "local" {
                entities.push((EntityType::Domain, clean.to_string()));
                continue;
            }
        }

        if clean == "powershell" || clean == "cmd" || clean == "rundll32" {
            entities.push((EntityType::Process, clean.to_string()));
            continue;
        }
    }

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
) -> Result<(StatusCode, Json<ApiResponse<IncidentSummary>>), AppError> {
    validate_upload(&req)?;

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
        let ct = parse_evidence_type(&e.content_type);
        let tecs = extract_mitre_techniques(&e.content);
        ext_techniques.extend(tecs);
        let ents = extract_entities_from_content(&e.content);
        ext_entities.extend(ents);
        evidence_list.push(Evidence::new(incident_id.clone(), e.source, e.content, ct, 0.9));
    }

    ext_techniques.sort();
    ext_techniques.dedup();
    incident.mitre_techniques = ext_techniques;

    ext_entities.sort_by_key(|e| (format!("{:?}", e.0), e.1.clone()));
    ext_entities.dedup_by(|a, b| format!("{:?}", a.0) == format!("{:?}", b.0) && a.1 == b.1);

    let mut entities = Vec::new();
    for (etype, ename) in ext_entities {
        let ent = Entity::new(ename, etype, serde_json::json!({}));
        entities.push(ent);
    }

    incident.entity_ids = entities.iter().map(|e| e.id.clone()).collect();
    incident.evidence_ids = evidence_list.iter().map(|e| e.id.clone()).collect();

    let severity_str = format_severity(&incident.severity);
    incident.tags = vec![severity_str.to_lowercase()];
    if !incident.mitre_techniques.is_empty() {
        incident.tags.push("mitre-mapped".to_string());
    }

    // Intelligence analysis
    if let Ok(mut intel) = state.intelligence.write() {
        let _ = intel.analyze(&evidence_list);
    }

    // Memory store
    let _ = state.memory.store_incident(&incident);

    // Persist to PgStore when available
    if let Some(ref pg) = state.pg_store {
        if let Err(e) = pg.save_incident(&incident).await {
            tracing::warn!("PgStore save_incident failed: {}", e);
        }
        if let Err(e) = pg.save_evidence_batch(&incident_id, &evidence_list).await {
            tracing::warn!("PgStore save_evidence failed: {}", e);
        }
        if let Err(e) = pg.save_entities_batch(&incident_id, &entities).await {
            tracing::warn!("PgStore save_entities failed: {}", e);
        }
    }

    // Qdrant upsert when available
    if let Some(ref qdrant) = state.qdrant {
        if let Some(ref ollama) = state.ollama_client {
            let combined_text = format!("{} {} {}", incident.title, incident.description,
                evidence_list.iter().map(|e| e.content.as_str()).collect::<Vec<_>>().join(" "));
            match ollama.generate_embedding(&combined_text).await {
                Ok(embedding) => {
                    let payload = serde_json::json!({
                        "incident_id": incident_id,
                        "title": incident.title,
                        "severity": severity_str,
                        "mitre_techniques": incident.mitre_techniques,
                        "evidence_count": evidence_list.len(),
                    });
                    if let Err(e) = qdrant.upsert_vector(&incident_id, embedding, payload).await {
                        tracing::warn!("Qdrant upsert failed: {}", e);
                    }
                }
                Err(e) => {
                    tracing::warn!("Embedding generation failed, skipping Qdrant: {}", e);
                }
            }
        }
    }

    // In-memory fallback
    let evidence_count = evidence_list.len();
    let entity_count = entities.len();

    tracing::info!(incident_id = %incident_id, evidence_count, entity_count, "Incident uploaded");

    if let Ok(mut incidents) = state.incidents.write() {
        incidents.insert(incident_id.clone(), incident);
    } else {
        return Err(AppError::LockError);
    }
    if let Ok(mut ev_map) = state.evidence.write() {
        ev_map.insert(incident_id.clone(), evidence_list);
    } else {
        return Err(AppError::LockError);
    }
    if let Ok(mut ent_map) = state.entities.write() {
        ent_map.insert(incident_id.clone(), entities);
    } else {
        return Err(AppError::LockError);
    }

    let summary = IncidentSummary {
        id: incident_id,
        title: req.title,
        severity: severity_str.to_string(),
        status: "New".to_string(),
        evidence_count,
        entity_count,
    };
    Ok(ApiResponse::created(summary))
}

pub async fn list_incidents(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<CanonicalIncident>>>), AppError> {
    if let Some(ref pg) = state.pg_store {
        match pg.list_incidents().await {
            Ok(incidents) => return Ok(ApiResponse::ok(incidents)),
            Err(e) => tracing::warn!("PgStore list_incidents failed, falling back: {}", e),
        }
    }
    match state.incidents.read() {
        Ok(incidents) => {
            let list: Vec<CanonicalIncident> = incidents.values().cloned().collect();
            Ok(ApiResponse::ok(list))
        }
        Err(_) => Err(AppError::LockError),
    }
}

pub async fn get_incident(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<ApiResponse<CanonicalIncident>>), AppError> {
    if id.is_empty() || id.len() > 128 {
        return Err(AppError::BadRequest("Invalid incident ID".into()));
    }
    if let Some(ref pg) = state.pg_store {
        match pg.get_incident(&id).await {
            Ok(Some(incident)) => return Ok(ApiResponse::ok(incident)),
            Ok(None) => {}
            Err(e) => tracing::warn!("PgStore get_incident failed, falling back: {}", e),
        }
    }
    match state.incidents.read() {
        Ok(incidents) => {
            match incidents.get(&id) {
                Some(incident) => Ok(ApiResponse::ok(incident.clone())),
                None => Err(AppError::NotFound("Incident not found".into())),
            }
        }
        Err(_) => Err(AppError::LockError),
    }
}

pub async fn get_timeline(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    if id.is_empty() || id.len() > 128 {
        return Err(AppError::BadRequest("Invalid incident ID".into()));
    }

    let ev_list = if let Some(ref pg) = state.pg_store {
        match pg.get_evidence(&id).await {
            Ok(evidence) => Some(evidence),
            Err(e) => {
                tracing::warn!("PgStore get_evidence failed, falling back: {}", e);
                None
            }
        }
    } else {
        None
    };

    let ev_list = ev_list.unwrap_or_else(|| {
        match state.evidence.read() {
            Ok(evidence) => evidence.get(&id).cloned().unwrap_or_default(),
            Err(_) => Vec::new(),
        }
    });

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

    Ok(ApiResponse::ok(serde_json::json!({ "incident_id": id, "events": timeline })))
}

pub async fn get_memory(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    if id.is_empty() || id.len() > 128 {
        return Err(AppError::BadRequest("Invalid incident ID".into()));
    }
    match state.memory.get_memory_by_incident(&id) {
        Ok(Some(memory)) => Ok(ApiResponse::ok(serde_json::json!(memory))),
        Ok(None) => Err(AppError::NotFound("Memory not found".into())),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}

pub async fn list_memories(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<MemoryObject>>>), AppError> {
    match state.memory.list_all() {
        Ok(memories) => Ok(ApiResponse::ok(memories)),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}

pub async fn search_similar(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SearchRequest>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    if req.incident_id.is_empty() || req.incident_id.len() > 128 {
        return Err(AppError::BadRequest("Invalid incident ID".into()));
    }
    let top_k = req.top_k.unwrap_or(5).clamp(1, 100);

    let query = if let Some(ref pg) = state.pg_store {
        match pg.get_incident(&req.incident_id).await {
            Ok(Some(i)) => i,
            _ => {
                match state.incidents.read() {
                    Ok(incidents) => match incidents.get(&req.incident_id) {
                        Some(i) => i.clone(),
                        None => return Err(AppError::NotFound("Incident not found".into())),
                    },
                    Err(_) => return Err(AppError::LockError),
                }
            }
        }
    } else {
        match state.incidents.read() {
            Ok(incidents) => match incidents.get(&req.incident_id) {
                Some(i) => i.clone(),
                None => return Err(AppError::NotFound("Incident not found".into())),
            },
            Err(_) => return Err(AppError::LockError),
        }
    };

    let candidates = state.memory.list_all().unwrap_or_default();

    let query_text = format!("{} {}", query.title, query.description);

    match state.retrieval.search_hybrid(&query, &candidates, &query_text, top_k).await {
        Ok(results) => Ok(ApiResponse::ok(serde_json::json!({ "results": results }))),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}

pub async fn search_text(
    State(state): State<Arc<AppState>>,
    Json(req): Json<TextSearchRequest>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    if req.query.trim().is_empty() {
        return Err(AppError::BadRequest("Search query is required".into()));
    }
    if req.query.len() > 5000 {
        return Err(AppError::BadRequest("Search query must be 5000 characters or fewer".into()));
    }
    let top_k = req.top_k.unwrap_or(5).clamp(1, 100);

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

    match state.retrieval.search_hybrid(&dummy_query, &candidates, &req.query, top_k).await {
        Ok(results) => Ok(ApiResponse::ok(serde_json::json!({ "results": results }))),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}

pub async fn generate_narrative(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    if id.is_empty() || id.len() > 128 {
        return Err(AppError::BadRequest("Invalid incident ID".into()));
    }

    let (summary, techniques) = if let Some(ref pg) = state.pg_store {
        match pg.get_incident(&id).await {
            Ok(Some(i)) => (format!("{}: {}", i.title, i.description), i.mitre_techniques.clone()),
            _ => {
                match state.incidents.read() {
                    Ok(incidents) => match incidents.get(&id) {
                        Some(i) => (format!("{}: {}", i.title, i.description), i.mitre_techniques.clone()),
                        None => return Err(AppError::NotFound("Incident not found".into())),
                    },
                    Err(_) => return Err(AppError::LockError),
                }
            }
        }
    } else {
        match state.incidents.read() {
            Ok(incidents) => match incidents.get(&id) {
                Some(i) => (format!("{}: {}", i.title, i.description), i.mitre_techniques.clone()),
                None => return Err(AppError::NotFound("Incident not found".into())),
            },
            Err(_) => return Err(AppError::LockError),
        }
    };

    let has_ollama = match state.intelligence.read() {
        Ok(intel) => intel.has_ollama(),
        Err(_) => return Err(AppError::LockError),
    };
    if !has_ollama {
        return Err(AppError::BadRequest("Ollama pipeline not configured".into()));
    }

    let ollama_client = state.ollama_client.clone()
        .ok_or_else(|| AppError::BadRequest("Ollama not configured".into()))?;
    let narrative = ollama_client.generate_narrative(&summary, &techniques)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(ApiResponse::ok(serde_json::json!({
        "incident_id": id,
        "narrative": narrative,
    })))
}

pub async fn get_graph(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    if id.is_empty() || id.len() > 128 {
        return Err(AppError::BadRequest("Invalid incident ID".into()));
    }
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    if let Ok(incidents) = state.incidents.read() {
        if let Some(inc) = incidents.get(&id) {
            nodes.push(serde_json::json!({
                "id": inc.id,
                "type": "incident",
                "label": inc.title,
            }));
        }
    }

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
        edges.push(serde_json::json!({
            "source": e.id,
            "target": id.clone(),
            "type": "evidence_of",
        }));
    }

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
        edges.push(serde_json::json!({
            "source": ent.id,
            "target": id.clone(),
            "type": "associated_with",
        }));
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

    Ok(ApiResponse::ok(serde_json::json!({
        "nodes": nodes,
        "edges": edges,
    })))
}

pub async fn get_global_graph(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
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

    Ok(ApiResponse::ok(serde_json::json!({ "nodes": nodes, "edges": edges })))
}

pub async fn get_playbooks(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    if id.is_empty() || id.len() > 128 {
        return Err(AppError::BadRequest("Invalid incident ID".into()));
    }
    let policy_results = state.policy.enforce(0.85);
    let policy_allowed = policy_results.iter().all(|r| {
        matches!(r.verdict, odin_core::odin_policy_gate::PolicyVerdict::Allow)
    });
    if !policy_allowed {
        return Err(AppError::BadRequest("Policy gate blocked".into()));
    }
    Ok(ApiResponse::ok(serde_json::json!({
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
    })))
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
) -> Result<(StatusCode, Json<ApiResponse<String>>), AppError> {
    if id.is_empty() || id.len() > 128 {
        return Err(AppError::BadRequest("Invalid incident ID".into()));
    }
    if req.feedback.len() > 5000 {
        return Err(AppError::BadRequest("Feedback must be 5000 characters or fewer".into()));
    }
    if req.rating > 5 {
        return Err(AppError::BadRequest("Rating must be between 0 and 5".into()));
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    // Persist to PgStore
    if let Some(ref pg) = state.pg_store {
        if let Err(e) = pg.save_feedback(&id, &req.feedback, req.rating).await {
            tracing::warn!("Failed to persist feedback: {}", e);
        }
    }

    // In-memory feedback store
    let entry = crate::state::FeedbackEntry {
        feedback: req.feedback,
        rating: req.rating,
        created_at: now,
    };
    if let Ok(mut feedback_map) = state.feedback.write() {
        feedback_map.entry(id.clone()).or_default().push(entry);
    }

    // Calculate average rating and update confidence signals
    let avg_rating = if let Some(ref pg) = state.pg_store {
        pg.get_average_rating(&id).await.unwrap_or(None).unwrap_or(req.rating as f64)
    } else {
        let entries = state.feedback.read()
            .map(|m| m.get(&id).cloned().unwrap_or_default())
            .unwrap_or_default();
        if entries.is_empty() {
            req.rating as f64
        } else {
            entries.iter().map(|e| e.rating as f64).sum::<f64>() / entries.len() as f64
        }
    };

    // Update retrieval feedback signal
    state.retrieval.set_feedback_signal(&id, avg_rating);

    tracing::info!(incident_id = %id, rating = req.rating, avg_rating, "Feedback recorded");
    Ok(ApiResponse::ok(format!("Feedback recorded for incident {} (avg rating: {:.1}/5.0)", id, avg_rating)))
}
