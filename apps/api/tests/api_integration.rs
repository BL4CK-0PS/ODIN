use odin_api::state::AppState;
use odin_api::worker::WorkerMetrics;
use odin_core::odin_decision_engine::DecisionEngine;
use odin_core::odin_infrastructure::{ArtifactStore, InfrastructureConfig};
use odin_core::odin_intelligence_engine::IntelligenceEngine;
use odin_core::odin_memory_engine::MemoryEngine;
use odin_core::odin_policy_gate::PolicyGate;
use odin_core::odin_retrieval_engine::RetrievalEngine;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

fn test_state() -> AppState {
    AppState {
        incidents: Arc::new(RwLock::new(HashMap::new())),
        evidence: Arc::new(RwLock::new(HashMap::new())),
        entities: Arc::new(RwLock::new(HashMap::new())),
        feedback: Arc::new(RwLock::new(HashMap::new())),
        memory: MemoryEngine::new(),
        intelligence: Arc::new(RwLock::new(IntelligenceEngine::new())),
        retrieval: RetrievalEngine::new(),
        decision: DecisionEngine::new(),
        policy: PolicyGate::new(),
        pg_store: None,
        qdrant: None,
        ollama_client: None,
        redis: None,
        neo4j: None,
        s3: None,
        artifact_store: ArtifactStore::new(None),
        worker_metrics: Arc::new(std::sync::RwLock::new(WorkerMetrics::default())),
        infra_config: InfrastructureConfig::from_env(),
    }
}

async fn start_server() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let state = Arc::new(test_state());
    let app = odin_api::build_router(state);
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let base = format!("http://{}", addr);
    let client = reqwest::Client::new();
    for _ in 0..50 {
        if client
            .get(format!("{}/api/v1/system/health", &base))
            .send()
            .await
            .is_ok()
        {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
    base
}

fn sample_incident() -> serde_json::Value {
    serde_json::json!({
        "title": "Ransomware Detection on File Server",
        "description": "Encrypting files detected on production file server",
        "severity": "critical",
        "evidence": [
            {
                "source": "edr",
                "content": "File encryption activity detected by process ransomware.exe",
                "content_type": "log"
            },
            {
                "source": "network",
                "content": "Outbound traffic to known C2 server 192.168.1.100",
                "content_type": "network_capture"
            }
        ]
    })
}

async fn upload_incident(base: &str) -> String {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/v1/incidents/upload", base))
        .json(&sample_incident())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);
    let json: serde_json::Value = resp.json().await.unwrap();
    json["data"]["id"].as_str().unwrap().to_string()
}

// ── Health & System ──────────────────────────────────────────────────────────

#[tokio::test]
async fn health_check() {
    let base = start_server().await;
    let resp = reqwest::get(format!("{}/api/v1/system/health", base))
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn version() {
    let base = start_server().await;
    let json: serde_json::Value = reqwest::get(format!("{}/api/v1/system/version", base))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(json["version"].as_str().is_some());
}

#[tokio::test]
async fn stats() {
    let base = start_server().await;
    let resp = reqwest::get(format!("{}/api/v1/system/stats", base))
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn metrics() {
    let base = start_server().await;
    let resp = reqwest::get(format!("{}/metrics", base)).await.unwrap();
    assert_eq!(resp.status(), 200);
}

// ── Incident CRUD ────────────────────────────────────────────────────────────

#[tokio::test]
async fn upload_incident_returns_created() {
    let base = start_server().await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/v1/incidents/upload", base))
        .json(&sample_incident())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 201);
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["success"], true);
    assert!(json["data"]["id"].as_str().is_some());
}

#[tokio::test]
async fn list_incidents_after_upload() {
    let base = start_server().await;
    let _ = upload_incident(&base).await;
    let json: serde_json::Value = reqwest::get(format!("{}/api/v1/incidents", base))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(json["data"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn get_incident_by_id() {
    let base = start_server().await;
    let id = upload_incident(&base).await;
    eprintln!("BASE: {}, ID: {}", base, id);
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let url = format!("{}/api/v1/incidents/{}", base, id);
    eprintln!("URL: {}", url);
    let resp = client.get(&url).send().await.unwrap();
    eprintln!("STATUS: {}", resp.status());
    let text = resp.text().await.unwrap();
    eprintln!("BODY: [{}]", text);
    assert!(!text.is_empty(), "Response body should not be empty");
    let json: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(json["data"]["title"], "Ransomware Detection on File Server");
}

#[tokio::test]
async fn get_nonexistent_incident() {
    let base = start_server().await;
    let resp = reqwest::get(format!("{}/api/v1/incidents/nonexistent", base))
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

// ── Incident Analysis ────────────────────────────────────────────────────────

#[tokio::test]
async fn timeline() {
    let base = start_server().await;
    let id = upload_incident(&base).await;
    let json: serde_json::Value =
        reqwest::get(format!("{}/api/v1/incidents/{}/timeline", base, id))
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
    let events = json["data"]["events"].as_array().unwrap();
    assert_eq!(events.len(), 2);
}

#[tokio::test]
async fn graph() {
    let base = start_server().await;
    let id = upload_incident(&base).await;
    let json: serde_json::Value = reqwest::get(format!("{}/api/v1/incidents/{}/graph", base, id))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(json["data"]["nodes"].as_array().unwrap().len() > 0);
}

#[tokio::test]
async fn memory() {
    let base = start_server().await;
    let id = upload_incident(&base).await;
    let resp = reqwest::get(format!("{}/api/v1/incidents/{}/memory", base, id))
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn predict() {
    let base = start_server().await;
    let id = upload_incident(&base).await;
    let json: serde_json::Value = reqwest::get(format!("{}/api/v1/incidents/{}/predict", base, id))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(
        json["data"]["prediction"]["recommended_steps"]
            .as_array()
            .unwrap()
            .len()
            > 0
    );
}

#[tokio::test]
async fn playbooks() {
    let base = start_server().await;
    let id = upload_incident(&base).await;
    let json: serde_json::Value =
        reqwest::get(format!("{}/api/v1/incidents/{}/playbooks", base, id))
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
    assert!(json["data"]["playbooks"].as_array().unwrap().len() > 0);
}

#[tokio::test]
async fn report_html() {
    let base = start_server().await;
    let id = upload_incident(&base).await;
    let text = reqwest::get(format!("{}/api/v1/incidents/{}/report", base, id))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(text.contains("Ransomware Detection"));
}

#[tokio::test]
async fn narrative_without_ollama() {
    let base = start_server().await;
    let id = upload_incident(&base).await;
    let resp = reqwest::get(format!("{}/api/v1/incidents/{}/narrative", base, id))
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);
}

// ── Feedback ─────────────────────────────────────────────────────────────────

#[tokio::test]
async fn post_feedback() {
    let base = start_server().await;
    let id = upload_incident(&base).await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/v1/incidents/{}/feedback", base, id))
        .json(&serde_json::json!({"feedback": "Great analysis", "rating": 5}))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn feedback_rating_too_high() {
    let base = start_server().await;
    let id = upload_incident(&base).await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/v1/incidents/{}/feedback", base, id))
        .json(&serde_json::json!({"feedback": "Great", "rating": 10}))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);
}

// ── Status ───────────────────────────────────────────────────────────────────

#[tokio::test]
async fn status_new_to_investigating() {
    let base = start_server().await;
    let id = upload_incident(&base).await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/v1/incidents/{}/status", base, id))
        .json(&serde_json::json!({"status": "investigating"}))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn status_invalid_transition() {
    let base = start_server().await;
    let id = upload_incident(&base).await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/v1/incidents/{}/status", base, id))
        .json(&serde_json::json!({"status": "recovered"}))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);
}

// ── Search ───────────────────────────────────────────────────────────────────

#[tokio::test]
async fn search_similar() {
    let base = start_server().await;
    let id = upload_incident(&base).await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/v1/incidents/search", base))
        .json(&serde_json::json!({"incident_id": id}))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn text_search() {
    let base = start_server().await;
    let _ = upload_incident(&base).await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/v1/search", base))
        .json(&serde_json::json!({"query": "file server"}))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
}

// ── Consolidation ────────────────────────────────────────────────────────────

#[tokio::test]
async fn consolidation_stats() {
    let base = start_server().await;
    let json: serde_json::Value = reqwest::get(format!("{}/api/v1/consolidation/stats", base))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(json["success"], true);
    assert!(json["data"]["total_memories"].as_u64().is_some());
}

// ── Memories & Graph ─────────────────────────────────────────────────────────

#[tokio::test]
async fn list_memories() {
    let base = start_server().await;
    let json: serde_json::Value = reqwest::get(format!("{}/api/v1/memories", base))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(json["data"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn global_graph() {
    let base = start_server().await;
    let json: serde_json::Value = reqwest::get(format!("{}/api/v1/graph", base))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(json["data"]["nodes"].is_array());
    assert!(json["data"]["edges"].is_array());
}

// ── Error Handling ───────────────────────────────────────────────────────────

#[tokio::test]
async fn invalid_json() {
    let base = start_server().await;
    let resp = reqwest::Client::new()
        .post(format!("{}/api/v1/incidents/upload", base))
        .header("content-type", "application/json")
        .body("not json")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn missing_required_fields() {
    let base = start_server().await;
    let resp = reqwest::Client::new()
        .post(format!("{}/api/v1/incidents/upload", base))
        .json(&serde_json::json!({"title": "Incomplete"}))
        .send()
        .await
        .unwrap();
    assert!(resp.status().is_client_error());
}

#[tokio::test]
async fn unknown_route() {
    let base = start_server().await;
    let resp = reqwest::get(format!("{}/api/v1/nonexistent", base))
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn wrong_http_method() {
    let base = start_server().await;
    let resp = reqwest::Client::new()
        .delete(format!("{}/api/v1/system/health", base))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 405);
}

// ── Multiple Incidents ───────────────────────────────────────────────────────

#[tokio::test]
async fn upload_multiple_incidents() {
    let base = start_server().await;
    for i in 0..3 {
        let incident = serde_json::json!({
            "title": format!("Incident {}", i),
            "severity": "high",
            "description": format!("Test incident {}", i),
            "evidence": [{"source": "edr", "content": format!("Malware on server-{}", i), "content_type": "log"}]
        });
        let resp = reqwest::Client::new()
            .post(format!("{}/api/v1/incidents/upload", base))
            .json(&incident)
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), 201);
    }
    let json: serde_json::Value = reqwest::get(format!("{}/api/v1/incidents", base))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(json["data"].as_array().unwrap().len(), 3);
}

// ── Knowledge (requires PgStore — expect 404 in-memory mode) ─────────────────

#[tokio::test]
async fn knowledge_list_requires_pgstore() {
    let base = start_server().await;
    let resp = reqwest::get(format!("{}/api/v1/knowledge/list", base))
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn knowledge_search_requires_pgstore() {
    let base = start_server().await;
    let resp = reqwest::Client::new()
        .post(format!("{}/api/v1/knowledge/search", base))
        .json(&serde_json::json!({"query": "APT29"}))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 404);
}
