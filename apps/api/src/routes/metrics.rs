use axum::response::{IntoResponse, Response};
use crate::state::AppState;
use std::sync::Arc;

pub async fn metrics(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
) -> Response {
    let mut lines = Vec::new();

    let investigations = state.incidents.read().map(|i| i.len()).unwrap_or(0);
    let memories = state.memory.list_all().map(|m| m.len()).unwrap_or(0);

    let entities = state.entities.read().map(|ents| {
        let mut unique = std::collections::HashSet::new();
        for list in ents.values() {
            for e in list {
                unique.insert(e.name.clone());
            }
        }
        unique.len()
    }).unwrap_or(0);

    let feedback_count = state.feedback.read().map(|f| {
        f.values().map(|v| v.len()).sum::<usize>()
    }).unwrap_or(0);

    lines.push("# HELP odin_investigations_total Total number of investigations.".to_string());
    lines.push("# TYPE odin_investigations_total gauge".to_string());
    lines.push(format!("odin_investigations_total {}", investigations));

    lines.push("# HELP odin_memories_total Total number of memory objects.".to_string());
    lines.push("# TYPE odin_memories_total gauge".to_string());
    lines.push(format!("odin_memories_total {}", memories));

    lines.push("# HELP odin_entities_total Total unique entities tracked.".to_string());
    lines.push("# TYPE odin_entities_total gauge".to_string());
    lines.push(format!("odin_entities_total {}", entities));

    lines.push("# HELP odin_feedback_total Total feedback submissions.".to_string());
    lines.push("# TYPE odin_feedback_total gauge".to_string());
    lines.push(format!("odin_feedback_total {}", feedback_count));

    lines.push("# HELP odin_up Whether the API is up (1 = up).".to_string());
    lines.push("# TYPE odin_up gauge".to_string());
    lines.push("odin_up 1".to_string());

    let pg_up = state.pg_store.is_some();
    lines.push("# HELP odin_postgres_connected Whether PostgreSQL is connected.".to_string());
    lines.push("# TYPE odin_postgres_connected gauge".to_string());
    lines.push(format!("odin_postgres_connected {}", if pg_up { 1 } else { 0 }));

    let qdrant_up = state.qdrant.is_some();
    lines.push("# HELP odin_qdrant_connected Whether Qdrant is connected.".to_string());
    lines.push("# TYPE odin_qdrant_connected gauge".to_string());
    lines.push(format!("odin_qdrant_connected {}", if qdrant_up { 1 } else { 0 }));

    let redis_up = state.redis.as_ref().map(|r| {
        tokio::runtime::Handle::try_current().ok().map(|h| {
            h.block_on(async { r.health_check().await })
        }).unwrap_or(false)
    }).unwrap_or(false);
    lines.push("# HELP odin_redis_connected Whether Redis is connected.".to_string());
    lines.push("# TYPE odin_redis_connected gauge".to_string());
    lines.push(format!("odin_redis_connected {}", if redis_up { 1 } else { 0 }));

    let neo4j_up = state.neo4j.as_ref().map(|n| {
        tokio::runtime::Handle::try_current().ok().map(|h| {
            h.block_on(async { n.health_check().await })
        }).unwrap_or(false)
    }).unwrap_or(false);
    lines.push("# HELP odin_neo4j_connected Whether Neo4j is connected.".to_string());
    lines.push("# TYPE odin_neo4j_connected gauge".to_string());
    lines.push(format!("odin_neo4j_connected {}", if neo4j_up { 1 } else { 0 }));

    let ollama_up = state.ollama_client.is_some();
    lines.push("# HELP odin_ollama_connected Whether Ollama is connected.".to_string());
    lines.push("# TYPE odin_ollama_connected gauge".to_string());
    lines.push(format!("odin_ollama_connected {}", if ollama_up { 1 } else { 0 }));

    let audit_stats = state.audit_logger.get_stats();
    lines.push("# HELP odin_audit_entries_total Total audit log entries.".to_string());
    lines.push("# TYPE odin_audit_entries_total gauge".to_string());
    lines.push(format!("odin_audit_entries_total {}", audit_stats.total_entries));

    lines.push("# HELP odin_audit_failed_total Failed audit attempts.".to_string());
    lines.push("# TYPE odin_audit_failed_total counter".to_string());
    lines.push(format!("odin_audit_failed_total {}", audit_stats.failed_attempts));

    let body = lines.join("\n") + "\n";

    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
        body,
    ).into_response()
}
