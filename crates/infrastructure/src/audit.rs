use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub action: AuditAction,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub details: serde_json::Value,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditAction {
    Login,
    Logout,
    Create,
    Read,
    Update,
    Delete,
    Search,
    Export,
    Transition,
    Feedback,
    Upload,
    Configure,
    AccessDenied,
}

pub struct AuditLogger {
    entries: RwLock<VecDeque<AuditEntry>>,
    max_entries: usize,
}

impl AuditLogger {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: RwLock::new(VecDeque::new()),
            max_entries,
        }
    }

    pub fn log(&self, entry: AuditEntry) {
        if let Ok(mut entries) = self.entries.write() {
            entries.push_back(entry);
            while entries.len() > self.max_entries {
                entries.pop_front();
            }
        }
    }

    pub fn get_entries(&self, limit: usize) -> Vec<AuditEntry> {
        if let Ok(entries) = self.entries.read() {
            entries.iter().rev().take(limit).cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_entries_for_user(&self, user_id: &str, limit: usize) -> Vec<AuditEntry> {
        if let Ok(entries) = self.entries.read() {
            entries.iter()
                .rev()
                .filter(|e| e.user_id.as_deref() == Some(user_id))
                .take(limit)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_entries_for_resource(&self, resource_type: &str, resource_id: &str, limit: usize) -> Vec<AuditEntry> {
        if let Ok(entries) = self.entries.read() {
            entries.iter()
                .rev()
                .filter(|e| e.resource_type == resource_type && e.resource_id.as_deref() == Some(resource_id))
                .take(limit)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_stats(&self) -> AuditStats {
        if let Ok(entries) = self.entries.read() {
            let total = entries.len();
            let failed = entries.iter().filter(|e| !e.success).count();
            let actions: std::collections::HashMap<String, usize> = entries.iter()
                .fold(std::collections::HashMap::new(), |mut acc, e| {
                    *acc.entry(format!("{:?}", e.action)).or_insert(0) += 1;
                    acc
                });
            AuditStats {
                total_entries: total,
                failed_attempts: failed,
                action_counts: actions,
            }
        } else {
            AuditStats::default()
        }
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new(10000)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditStats {
    pub total_entries: usize,
    pub failed_attempts: usize,
    pub action_counts: std::collections::HashMap<String, usize>,
}
