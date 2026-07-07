use crate::{Confidence, ConfidenceSource, IntelligenceObject, KernelError, Provenance, SourceType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryObject {
    pub id: String,
    pub incident_id: String,
    pub summary: String,
    pub context: serde_json::Value,
    pub confidence: f64,
    pub version: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl MemoryObject {
    pub fn new(
        incident_id: String,
        summary: String,
        context: serde_json::Value,
        confidence: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            incident_id,
            summary,
            context,
            confidence: confidence.clamp(0.0, 1.0),
            version: 1,
            created_at: chrono::Utc::now(),
            expires_at: None,
        }
    }
}

impl IntelligenceObject for MemoryObject {
    fn id(&self) -> &str {
        &self.id
    }

    fn object_type(&self) -> &'static str {
        "memory_object"
    }

    fn validate(&self) -> Result<(), KernelError> {
        if self.summary.trim().is_empty() {
            return Err(KernelError::Validation(
                "memory object summary must not be empty".into(),
            ));
        }
        if !(0.0..=1.0).contains(&self.confidence) {
            return Err(KernelError::Validation(
                "confidence must be between 0.0 and 1.0".into(),
            ));
        }
        Ok(())
    }

    fn confidence(&self) -> Confidence {
        Confidence::new(vec![ConfidenceSource {
            label: format!("memory:{}", self.id),
            trust: self.confidence,
        }])
    }

    fn provenance(&self) -> Provenance {
        Provenance::new(
            format!("memory:{}", self.incident_id),
            SourceType::Other("memory".into()),
            "system".into(),
        )
    }
}
