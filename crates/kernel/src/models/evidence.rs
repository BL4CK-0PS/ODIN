use crate::{Confidence, ConfidenceSource, IntelligenceObject, KernelError, Provenance, SourceType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: String,
    pub incident_id: String,
    pub source: String,
    pub content: String,
    pub content_type: EvidenceType,
    pub trust_score: f64,
    pub collected_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    Log,
    NetworkCapture,
    FileSystemArtifact,
    MemoryDump,
    ThreatIntelReport,
    UserReport,
    Other(String),
}

impl Evidence {
    pub fn new(
        incident_id: String,
        source: String,
        content: String,
        content_type: EvidenceType,
        trust_score: f64,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            incident_id,
            source,
            content,
            content_type,
            trust_score: trust_score.clamp(0.0, 1.0),
            collected_at: now,
            created_at: now,
        }
    }
}

impl IntelligenceObject for Evidence {
    fn id(&self) -> &str {
        &self.id
    }

    fn object_type(&self) -> &'static str {
        "evidence"
    }

    fn validate(&self) -> Result<(), KernelError> {
        if self.source.trim().is_empty() {
            return Err(KernelError::Validation(
                "evidence source must not be empty".into(),
            ));
        }
        if !(0.0..=1.0).contains(&self.trust_score) {
            return Err(KernelError::Validation(
                "trust_score must be between 0.0 and 1.0".into(),
            ));
        }
        Ok(())
    }

    fn confidence(&self) -> Confidence {
        Confidence::new(vec![ConfidenceSource {
            label: format!("evidence:{}", self.id),
            trust: self.trust_score,
        }])
    }

    fn provenance(&self) -> Provenance {
        Provenance::new(self.source.clone(), SourceType::Other("evidence".into()), "system".into())
    }
}
