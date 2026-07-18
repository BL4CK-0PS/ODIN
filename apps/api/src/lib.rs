pub mod error;
pub mod report;
pub mod response;
pub mod routes;
pub mod state;
pub mod worker;

use axum::{
    routing::{get, post},
    Router,
};
use routes::{incidents, knowledge, metrics, system};
use state::AppState;
use std::sync::Arc;
use tower::limit::ConcurrencyLimitLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::TraceLayer;

pub fn build_router(state: Arc<AppState>) -> Router {
    let cors_origins = std::env::var("ODIN_CORS_ORIGINS").unwrap_or_else(|_| "*".to_string());
    let cors_layer = if cors_origins == "*" {
        CorsLayer::permissive()
    } else {
        let origins: Vec<_> = cors_origins
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        CorsLayer::new()
            .allow_origin(
                origins
                    .iter()
                    .map(|s| s.parse::<axum::http::HeaderValue>().unwrap())
                    .collect::<Vec<_>>(),
            )
            .allow_methods(Any)
            .allow_headers(Any)
    };

    let incident_routes = Router::new()
        .route("/", get(incidents::list_incidents))
        .route("/upload", post(incidents::upload))
        .route("/search", post(incidents::search_similar))
        .route("/:id", get(incidents::get_incident))
        .route("/:id/timeline", get(incidents::get_timeline))
        .route("/:id/graph", get(incidents::get_graph))
        .route("/:id/memory", get(incidents::get_memory))
        .route("/:id/playbooks", get(incidents::get_playbooks))
        .route("/:id/predict", get(incidents::predict_next_steps))
        .route("/:id/feedback", post(incidents::post_feedback))
        .route("/:id/narrative", get(incidents::generate_narrative))
        .route("/:id/report", get(incidents::generate_report))
        .route("/:id/status", post(incidents::update_status));

    let knowledge_routes = Router::new()
        .route("/", post(knowledge::create_knowledge_object))
        .route("/search", post(knowledge::search_knowledge_objects))
        .route("/list", get(knowledge::list_knowledge_objects))
        .route(
            "/:id",
            get(knowledge::get_knowledge_object).post(knowledge::update_knowledge_object),
        )
        .route(
            "/:id/transition",
            post(knowledge::transition_knowledge_object),
        )
        .route("/:id/delete", post(knowledge::delete_knowledge_object));

    Router::new()
        .nest("/api/v1/incidents", incident_routes)
        .route("/api/v1/memories", get(incidents::list_memories))
        .route("/api/v1/search", post(incidents::search_text))
        .route("/api/v1/graph", get(incidents::get_global_graph))
        .route(
            "/api/v1/consolidation/stats",
            get(incidents::get_consolidation_stats),
        )
        .nest("/api/v1/knowledge", knowledge_routes)
        .route("/api/v1/system/health", get(system::health))
        .route("/api/v1/system/version", get(system::version))
        .route("/api/v1/system/stats", get(system::stats))
        .route("/metrics", get(metrics::metrics))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(tower_http::trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                ),
        )
        .layer(cors_layer)
        .layer(SetRequestIdLayer::new(
            axum::http::HeaderName::from_static("x-request-id"),
            MakeRequestUuid,
        ))
        .layer(ConcurrencyLimitLayer::new(
            std::env::var("ODIN_MAX_CONCURRENCY")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(64),
        ))
        .with_state(state)
}
