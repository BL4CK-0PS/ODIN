use crate::{Confidence, ConfidenceSource, IntelligenceObject, KernelError, Provenance, SourceType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub entity_type: EntityType,
    pub metadata: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    IpAddress,
    Domain,
    Hash,
    Hostname,
    User,
    Process,
    File,
    NetworkConnection,
    Artifact,
    Other(String),
}

impl Entity {
    pub fn new(name: String, entity_type: EntityType, metadata: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            entity_type,
            metadata,
            created_at: chrono::Utc::now(),
        }
    }
}

impl IntelligenceObject for Entity {
    fn id(&self) -> &str {
        &self.id
    }

    fn object_type(&self) -> &'static str {
        "entity"
    }

    fn validate(&self) -> Result<(), KernelError> {
        if self.name.trim().is_empty() {
            return Err(KernelError::Validation(
                "entity name must not be empty".into(),
            ));
        }
        Ok(())
    }

    fn confidence(&self) -> Confidence {
        Confidence::new(vec![ConfidenceSource {
            label: format!("entity:{}", self.id),
            trust: 0.97,
        }])
    }

    fn provenance(&self) -> Provenance {
        Provenance::new(self.name.clone(), SourceType::Other("entity".into()), "system".into())
    }
}
