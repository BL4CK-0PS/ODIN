use crate::{Confidence, ConfidenceSource, IntelligenceObject, KernelError, Provenance, SourceType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalIncident {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub status: IncidentStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub tags: Vec<String>,
    pub evidence_ids: Vec<String>,
    pub entity_ids: Vec<String>,
    pub mitre_techniques: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IncidentStatus {
    New,
    Investigating,
    Contained,
    Eradicated,
    Recovered,
    Closed,
}

impl CanonicalIncident {
    pub fn new(title: String, description: String, severity: Severity) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            severity,
            status: IncidentStatus::New,
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
            evidence_ids: Vec::new(),
            entity_ids: Vec::new(),
            mitre_techniques: Vec::new(),
        }
    }
}

impl IntelligenceObject for CanonicalIncident {
    fn id(&self) -> &str {
        &self.id
    }

    fn object_type(&self) -> &'static str {
        "canonical_incident"
    }

    fn validate(&self) -> Result<(), KernelError> {
        if self.title.trim().is_empty() {
            return Err(KernelError::Validation(
                "title must not be empty".into(),
            ));
        }
        if self.description.trim().is_empty() {
            return Err(KernelError::Validation(
                "description must not be empty".into(),
            ));
        }
        Ok(())
    }

    fn confidence(&self) -> Confidence {
        Confidence::new(vec![ConfidenceSource {
            label: format!("incident:{}", self.id),
            trust: 1.0,
        }])
    }

    fn provenance(&self) -> Provenance {
        Provenance::new(self.title.clone(), SourceType::Other("incident".into()), "system".into())
    }
}
