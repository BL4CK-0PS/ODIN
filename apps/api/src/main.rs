mod error;
mod response;
mod routes;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use routes::incidents;
use routes::system;
use state::AppState;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    let state = Arc::new(AppState::new());

    let cors_origins = std::env::var("ODIN_CORS_ORIGINS").unwrap_or_else(|_| "*".to_string());
    let cors_layer = if cors_origins == "*" {
        CorsLayer::permissive()
    } else {
        let origins: Vec<_> = cors_origins.split(',').map(|s| s.trim().to_string()).collect();
        CorsLayer::new()
            .allow_origin(origins.iter().map(|s| s.parse::<axum::http::HeaderValue>().unwrap()).collect::<Vec<_>>())
            .allow_methods(Any)
            .allow_headers(Any)
    };

    let app = Router::new()
        .route("/api/v1/system/health", get(system::health))
        .route("/api/v1/system/version", get(system::version))
        .route("/api/v1/system/stats", get(system::stats))
        .route("/api/v1/incidents", get(incidents::list_incidents))
        .route("/api/v1/incidents/upload", post(incidents::upload))
        .route("/api/v1/incidents/{id}", get(incidents::get_incident))
        .route("/api/v1/incidents/search", post(incidents::search_similar))
        .route("/api/v1/incidents/{id}/timeline", get(incidents::get_timeline))
        .route("/api/v1/incidents/{id}/graph", get(incidents::get_graph))
        .route("/api/v1/incidents/{id}/memory", get(incidents::get_memory))
        .route("/api/v1/incidents/{id}/playbooks", get(incidents::get_playbooks))
        .route("/api/v1/incidents/{id}/feedback", post(incidents::post_feedback))
        .route("/api/v1/memories", get(incidents::list_memories))
        .route("/api/v1/search", post(incidents::search_text))
        .route("/api/v1/graph", get(incidents::get_global_graph))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(tower_http::trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .layer(cors_layer)
        .layer(SetRequestIdLayer::new(
            axum::http::HeaderName::from_static("x-request-id"),
            MakeRequestUuid::default(),
        ))
        .with_state(state);

    let port = std::env::var("ODIN_PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");

    tracing::info!("ODIN API listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received, gracefully shutting down...");
}
