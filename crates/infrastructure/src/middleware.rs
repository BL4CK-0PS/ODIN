use crate::auth::{Claims, Permission, Role, User};
use crate::jwt::JwtService;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user: User,
    pub claims: Claims,
}

#[derive(Debug, Clone)]
pub struct AuthState {
    pub jwt_service: Arc<JwtService>,
}

impl AuthState {
    pub fn new(jwt_service: JwtService) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service),
        }
    }
}

pub async fn auth_middleware(
    State(auth_state): State<AuthState>,
    mut request: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    let auth_header = request.headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));

    match auth_header {
        Some(token) => {
            match auth_state.jwt_service.validate_token(token) {
                Ok(claims) => {
                    let user = User {
                        id: claims.sub.clone(),
                        username: claims.username.clone(),
                        email: format!("{}@odin.local", claims.username),
                        roles: claims.roles.iter().filter_map(|r| match r.as_str() {
                            "Admin" => Some(Role::Admin),
                            "Analyst" => Some(Role::Analyst),
                            "Viewer" => Some(Role::Viewer),
                            "ReadOnly" => Some(Role::ReadOnly),
                            _ => None,
                        }).collect(),
                        created_at: chrono::Utc::now(),
                        last_login: None,
                        is_active: true,
                    };

                    let auth_user = AuthenticatedUser { user, claims };
                    request.extensions_mut().insert(auth_user);
                    Ok(next.run(request).await)
                }
                Err(_) => Err(axum::http::StatusCode::UNAUTHORIZED),
            }
        }
        None => Err(axum::http::StatusCode::UNAUTHORIZED),
    }
}

pub fn require_permission(permission: Permission) -> impl Fn(axum::http::request::Parts) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), axum::http::StatusCode>> + Send>> + Clone {
    move |parts: axum::http::request::Parts| {
        let perm = permission.clone();
        Box::pin(async move {
            if let Some(auth_user) = parts.extensions.get::<AuthenticatedUser>() {
                if auth_user.user.has_permission(&perm) {
                    Ok(())
                } else {
                    Err(axum::http::StatusCode::FORBIDDEN)
                }
            } else {
                Err(axum::http::StatusCode::UNAUTHORIZED)
            }
        })
    }
}
