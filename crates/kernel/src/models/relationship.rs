use crate::{IntelligenceObject, KernelError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: RelationshipType,
    pub confidence: f64,
    pub metadata: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    RelatedTo,
    DerivedFrom,
    References,
    PartOf,
    Mitigates,
    Exploits,
    Indicates,
    Other(String),
}

impl Relationship {
    pub fn new(
        source_id: String,
        target_id: String,
        relationship_type: RelationshipType,
        confidence: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            source_id,
            target_id,
            relationship_type,
            confidence: confidence.clamp(0.0, 1.0),
            metadata: serde_json::Value::Object(Default::default()),
            created_at: chrono::Utc::now(),
        }
    }
}

impl IntelligenceObject for Relationship {
    fn id(&self) -> &str {
        &self.id
    }

    fn object_type(&self) -> &'static str {
        "relationship"
    }

    fn validate(&self) -> Result<(), KernelError> {
        if self.source_id.is_empty() || self.target_id.is_empty() {
            return Err(KernelError::Validation(
                "source_id and target_id must not be empty".into(),
            ));
        }
        if !(0.0..=1.0).contains(&self.confidence) {
            return Err(KernelError::Validation(
                "confidence must be between 0.0 and 1.0".into(),
            ));
        }
        Ok(())
    }
}
