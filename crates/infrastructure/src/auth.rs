use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Role {
    Admin,
    Analyst,
    Viewer,
    ReadOnly,
}

impl Role {
    pub fn permissions(&self) -> Vec<Permission> {
        match self {
            Role::Admin => vec![
                Permission::ReadIncidents,
                Permission::WriteIncidents,
                Permission::DeleteIncidents,
                Permission::ManageUsers,
                Permission::ManageKnowledge,
                Permission::ViewAuditLogs,
                Permission::ExportData,
                Permission::ConfigureSystem,
            ],
            Role::Analyst => vec![
                Permission::ReadIncidents,
                Permission::WriteIncidents,
                Permission::ManageKnowledge,
                Permission::ViewAuditLogs,
                Permission::ExportData,
            ],
            Role::Viewer => vec![
                Permission::ReadIncidents,
                Permission::ViewAuditLogs,
            ],
            Role::ReadOnly => vec![
                Permission::ReadIncidents,
            ],
        }
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions().contains(permission)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Permission {
    ReadIncidents,
    WriteIncidents,
    DeleteIncidents,
    ManageUsers,
    ManageKnowledge,
    ViewAuditLogs,
    ExportData,
    ConfigureSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<Role>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
}

impl User {
    pub fn new(username: String, email: String, roles: Vec<Role>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            username,
            email,
            roles,
            created_at: chrono::Utc::now(),
            last_login: None,
            is_active: true,
        }
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.roles.iter().any(|role| role.has_permission(permission))
    }

    pub fn has_role(&self, role: &Role) -> bool {
        self.roles.contains(role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role(&Role::Admin)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub roles: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}
