use crate::error::AppError;
use crate::response::ApiResponse;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use odin_core::odin_kernel::{KnowledgeObject, KnowledgeStatus, KnowledgeType};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CreateKnowledgeRequest {
    pub title: String,
    pub description: String,
    pub content: String,
    pub object_type: String,
    pub tags: Option<Vec<String>>,
    pub source_incidents: Option<Vec<String>>,
    pub mitre_techniques: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateKnowledgeRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
    pub mitre_techniques: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct TransitionRequest {
    pub status: String,
    pub reason: String,
    pub actor: String,
}

#[derive(Debug, Deserialize)]
pub struct ListKnowledgeQuery {
    pub status: Option<String>,
    pub object_type: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub async fn create_knowledge_object(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateKnowledgeRequest>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    if req.title.is_empty() {
        return Err(AppError::BadRequest("Title is required".into()));
    }

    let object_type = match req.object_type.as_str() {
        "Playbook" => KnowledgeType::Playbook,
        "ThreatIntel" => KnowledgeType::ThreatIntel,
        "MitreMapping" => KnowledgeType::MitreMapping,
        "IocDefinition" => KnowledgeType::IocDefinition,
        "ResponseProcedure" => KnowledgeType::ResponseProcedure,
        "Policy" => KnowledgeType::Policy,
        other => KnowledgeType::Custom(other.to_string()),
    };

    let mut obj = KnowledgeObject::new(
        req.title,
        req.description,
        req.content,
        object_type,
        "system".to_string(),
    );

    if let Some(tags) = req.tags {
        obj.tags = tags;
    }
    if let Some(incidents) = req.source_incidents {
        obj.source_incidents = incidents;
    }
    if let Some(techniques) = req.mitre_techniques {
        obj.mitre_techniques = techniques;
    }

    if let Some(ref pg) = state.pg_store {
        pg.save_knowledge_object(&obj)
            .await
            .map_err(|e| AppError::BadRequest(format!("Failed to save: {}", e)))?;
    }

    let id = obj.id.clone();
    Ok(ApiResponse::created(serde_json::json!({
        "id": id,
        "title": obj.title,
        "status": format!("{:?}", obj.status),
        "object_type": format!("{:?}", obj.object_type),
        "created_at": obj.created_at,
    })))
}

pub async fn get_knowledge_object(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    if id.is_empty() || id.len() > 128 {
        return Err(AppError::BadRequest("Invalid ID".into()));
    }

    let obj = if let Some(ref pg) = state.pg_store {
        pg.get_knowledge_object(&id)
            .await
            .map_err(|e| AppError::BadRequest(format!("Failed to get: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Knowledge object not found".into()))?
    } else {
        return Err(AppError::NotFound("Knowledge store not available".into()));
    };

    Ok(ApiResponse::ok(serde_json::json!({
        "id": obj.id,
        "title": obj.title,
        "description": obj.description,
        "content": obj.content,
        "object_type": format!("{:?}", obj.object_type),
        "status": format!("{:?}", obj.status),
        "tags": obj.tags,
        "source_incidents": obj.source_incidents,
        "mitre_techniques": obj.mitre_techniques,
        "created_by": obj.created_by,
        "updated_by": obj.updated_by,
        "created_at": obj.created_at,
        "updated_at": obj.updated_at,
        "available_transitions": obj.available_transitions().iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>(),
        "is_editable": obj.is_editable(),
        "review_notes": obj.review_notes,
    })))
}

pub async fn update_knowledge_object(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateKnowledgeRequest>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    if id.is_empty() || id.len() > 128 {
        return Err(AppError::BadRequest("Invalid ID".into()));
    }

    let mut obj = if let Some(ref pg) = state.pg_store {
        pg.get_knowledge_object(&id)
            .await
            .map_err(|e| AppError::BadRequest(format!("Failed to get: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Knowledge object not found".into()))?
    } else {
        return Err(AppError::NotFound("Knowledge store not available".into()));
    };

    if !obj.is_editable() {
        return Err(AppError::BadRequest(
            "Object is not in an editable state".into(),
        ));
    }

    if let Some(title) = req.title {
        obj.title = title;
    }
    if let Some(description) = req.description {
        obj.description = description;
    }
    if let Some(content) = req.content {
        obj.content = content;
    }
    if let Some(tags) = req.tags {
        obj.tags = tags;
    }
    if let Some(techniques) = req.mitre_techniques {
        obj.mitre_techniques = techniques;
    }
    obj.updated_at = chrono::Utc::now();

    if let Some(ref pg) = state.pg_store {
        pg.save_knowledge_object(&obj)
            .await
            .map_err(|e| AppError::BadRequest(format!("Failed to update: {}", e)))?;
    }

    Ok(ApiResponse::ok(serde_json::json!({
        "id": obj.id,
        "title": obj.title,
        "status": format!("{:?}", obj.status),
        "updated_at": obj.updated_at,
    })))
}

pub async fn transition_knowledge_object(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<TransitionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    if id.is_empty() || id.len() > 128 {
        return Err(AppError::BadRequest("Invalid ID".into()));
    }

    let mut obj = if let Some(ref pg) = state.pg_store {
        pg.get_knowledge_object(&id)
            .await
            .map_err(|e| AppError::BadRequest(format!("Failed to get: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Knowledge object not found".into()))?
    } else {
        return Err(AppError::NotFound("Knowledge store not available".into()));
    };

    let new_status = match req.status.as_str() {
        "Review" => KnowledgeStatus::Review,
        "Active" => KnowledgeStatus::Active,
        "Deprecated" => KnowledgeStatus::Deprecated,
        "Archived" => KnowledgeStatus::Archived,
        "Purged" => KnowledgeStatus::Purged,
        "Draft" => KnowledgeStatus::Draft,
        _ => return Err(AppError::BadRequest("Invalid status".into())),
    };

    obj.transition(new_status, &req.reason, &req.actor)
        .map_err(|e| AppError::BadRequest(format!("Transition failed: {}", e)))?;

    if let Some(ref pg) = state.pg_store {
        pg.save_knowledge_object(&obj)
            .await
            .map_err(|e| AppError::BadRequest(format!("Failed to save transition: {}", e)))?;
    }

    Ok(ApiResponse::ok(serde_json::json!({
        "id": obj.id,
        "status": format!("{:?}", obj.status),
        "available_transitions": obj.available_transitions().iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>(),
        "status_history": obj.status_history.iter().map(|t| serde_json::json!({
            "from": format!("{:?}", t.from),
            "to": format!("{:?}", t.to),
            "reason": t.reason,
            "actor": t.actor,
            "timestamp": t.timestamp,
        })).collect::<Vec<_>>(),
    })))
}

pub async fn list_knowledge_objects(
    State(state): State<Arc<AppState>>,
    axum::extract::Query(query): axum::extract::Query<ListKnowledgeQuery>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    let limit = query.limit.unwrap_or(50).min(200);
    let offset = query.offset.unwrap_or(0);

    let objects = if let Some(ref pg) = state.pg_store {
        pg.list_knowledge_objects(
            query.status.as_deref(),
            query.object_type.as_deref(),
            limit,
            offset,
        )
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to list: {}", e)))?
    } else {
        return Err(AppError::NotFound("Knowledge store not available".into()));
    };

    let items: Vec<serde_json::Value> = objects
        .iter()
        .map(|obj| {
            serde_json::json!({
                "id": obj.id,
                "title": obj.title,
                "description": obj.description,
                "object_type": format!("{:?}", obj.object_type),
                "status": format!("{:?}", obj.status),
                "tags": obj.tags,
                "created_by": obj.created_by,
                "updated_at": obj.updated_at,
            })
        })
        .collect();

    Ok(ApiResponse::ok(serde_json::json!({
        "items": items,
        "count": items.len(),
        "limit": limit,
        "offset": offset,
    })))
}

pub async fn delete_knowledge_object(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<ApiResponse<String>>), AppError> {
    if id.is_empty() || id.len() > 128 {
        return Err(AppError::BadRequest("Invalid ID".into()));
    }

    if let Some(ref pg) = state.pg_store {
        pg.delete_knowledge_object(&id)
            .await
            .map_err(|e| AppError::BadRequest(format!("Failed to delete: {}", e)))?;
    }

    Ok(ApiResponse::ok("Deleted".to_string()))
}

pub async fn search_knowledge_objects(
    State(state): State<Arc<AppState>>,
    Json(req): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    let query_text = req.get("query").and_then(|v| v.as_str()).unwrap_or("");
    let limit = req.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as usize;

    if query_text.is_empty() {
        return Err(AppError::BadRequest("Query is required".into()));
    }

    let objects = if let Some(ref pg) = state.pg_store {
        pg.search_knowledge_objects(query_text, limit)
            .await
            .map_err(|e| AppError::BadRequest(format!("Search failed: {}", e)))?
    } else {
        return Err(AppError::NotFound("Knowledge store not available".into()));
    };

    let items: Vec<serde_json::Value> = objects
        .iter()
        .map(|obj| {
            serde_json::json!({
                "id": obj.id,
                "title": obj.title,
                "description": obj.description,
                "object_type": format!("{:?}", obj.object_type),
                "status": format!("{:?}", obj.status),
                "tags": obj.tags,
            })
        })
        .collect();

    Ok(ApiResponse::ok(serde_json::json!({
        "items": items,
        "count": items.len(),
    })))
}
