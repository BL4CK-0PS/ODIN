use odin_kernel::MemoryObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryVersion {
    pub version: u64,
    pub memory: MemoryObject,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub changelog: String,
}

impl MemoryVersion {
    pub fn new(memory: MemoryObject, changelog: String) -> Self {
        let version = memory.version;
        Self {
            version,
            memory,
            created_at: chrono::Utc::now(),
            changelog,
        }
    }
}
