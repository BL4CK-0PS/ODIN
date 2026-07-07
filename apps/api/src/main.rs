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
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/api/v1/system/health", get(system::health))
        .route("/api/v1/system/version", get(system::version))
        .route("/api/v1/incidents/upload", post(incidents::upload))
        .route("/api/v1/incidents/{id}", get(incidents::get_incident))
        .route("/api/v1/incidents/search", post(incidents::search_similar))
        .route("/api/v1/incidents/{id}/timeline", get(incidents::get_timeline))
        .route("/api/v1/incidents/{id}/graph", get(incidents::get_graph))
        .route("/api/v1/incidents/{id}/memory", get(incidents::get_memory))
        .route("/api/v1/incidents/{id}/playbooks", get(incidents::get_playbooks))
        .route("/api/v1/incidents/{id}/feedback", post(incidents::post_feedback))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind");

    tracing::info!("ODIN API listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
