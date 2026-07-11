use crate::error::AppError;
use crate::response::ApiResponse;
use crate::state::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use odin_core::odin_infrastructure::{User, Role};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AuditQuery {
    pub limit: Option<usize>,
    pub user_id: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    if req.username.is_empty() || req.password.is_empty() {
        return Err(AppError::BadRequest("Username and password required".into()));
    }

    if req.password != "password" {
        state.audit_logger.log(odin_core::odin_infrastructure::AuditEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            user_id: None,
            username: Some(req.username.clone()),
            action: odin_core::odin_infrastructure::AuditAction::Login,
            resource_type: "auth".to_string(),
            resource_id: None,
            details: serde_json::json!({"success": false}),
            ip_address: None,
            user_agent: None,
            success: false,
            error_message: Some("Invalid credentials".to_string()),
        });
        return Err(AppError::Unauthorized("Invalid credentials".into()));
    }

    let roles = if req.username == "admin" {
        vec![Role::Admin]
    } else {
        vec![Role::Analyst]
    };

    let user = User::new(req.username.clone(), format!("{}@odin.local", req.username), roles);

    let token = state.jwt_service.generate_token(&user)
        .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))?;

    state.audit_logger.log(odin_core::odin_infrastructure::AuditEntry {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now(),
        user_id: Some(user.id.clone()),
        username: Some(req.username.clone()),
        action: odin_core::odin_infrastructure::AuditAction::Login,
        resource_type: "auth".to_string(),
        resource_id: None,
        details: serde_json::json!({"success": true}),
        ip_address: None,
        user_agent: None,
        success: true,
        error_message: None,
    });

    Ok(ApiResponse::ok(serde_json::json!({
        "token": token,
        "user": {
            "id": user.id,
            "username": user.username,
            "roles": user.roles.iter().map(|r| format!("{:?}", r)).collect::<Vec<_>>(),
        },
    })))
}

pub async fn get_audit_logs(
    State(state): State<Arc<AppState>>,
    axum::extract::Query(query): axum::extract::Query<AuditQuery>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    let limit = query.limit.unwrap_or(100).min(1000);

    let entries = if let Some(user_id) = &query.user_id {
        state.audit_logger.get_entries_for_user(user_id, limit)
    } else if let (Some(resource_type), Some(resource_id)) = (&query.resource_type, &query.resource_id) {
        state.audit_logger.get_entries_for_resource(resource_type, resource_id, limit)
    } else {
        state.audit_logger.get_entries(limit)
    };

    let items: Vec<serde_json::Value> = entries.iter().map(|e| serde_json::json!({
        "id": e.id,
        "timestamp": e.timestamp,
        "user_id": e.user_id,
        "username": e.username,
        "action": format!("{:?}", e.action),
        "resource_type": e.resource_type,
        "resource_id": e.resource_id,
        "success": e.success,
        "error_message": e.error_message,
    })).collect();

    Ok(ApiResponse::ok(serde_json::json!({
        "items": items,
        "count": items.len(),
    })))
}

pub async fn get_audit_stats(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    let stats = state.audit_logger.get_stats();
    Ok(ApiResponse::ok(serde_json::json!({
        "total_entries": stats.total_entries,
        "failed_attempts": stats.failed_attempts,
        "action_counts": stats.action_counts,
    })))
}
