use crate::{
    Confidence, ConfidenceSource, IntelligenceObject, KernelError, Provenance, SourceType,
};
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
            return Err(KernelError::Validation("title must not be empty".into()));
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
        Provenance::new(
            self.title.clone(),
            SourceType::Other("incident".into()),
            "system".into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_incident_has_uuid_and_defaults() {
        let inc = CanonicalIncident::new("Test".into(), "Desc".into(), Severity::High);
        assert!(!inc.id.is_empty());
        assert_eq!(inc.title, "Test");
        assert_eq!(inc.description, "Desc");
        assert!(matches!(inc.severity, Severity::High));
        assert!(matches!(inc.status, IncidentStatus::New));
        assert!(inc.tags.is_empty());
        assert!(inc.evidence_ids.is_empty());
        assert!(inc.entity_ids.is_empty());
        assert!(inc.mitre_techniques.is_empty());
    }

    #[test]
    fn validate_rejects_empty_title() {
        let inc = CanonicalIncident::new("".into(), "Desc".into(), Severity::Low);
        assert!(inc.validate().is_err());
    }

    #[test]
    fn validate_rejects_whitespace_only_title() {
        let inc = CanonicalIncident::new("   ".into(), "Desc".into(), Severity::Low);
        assert!(inc.validate().is_err());
    }

    #[test]
    fn validate_rejects_empty_description() {
        let inc = CanonicalIncident::new("Title".into(), "".into(), Severity::Low);
        assert!(inc.validate().is_err());
    }

    #[test]
    fn validate_accepts_valid_incident() {
        let inc = CanonicalIncident::new("Title".into(), "Description".into(), Severity::Medium);
        assert!(inc.validate().is_ok());
    }

    #[test]
    fn confidence_is_one_for_incident() {
        let inc = CanonicalIncident::new("T".into(), "D".into(), Severity::Low);
        assert_eq!(inc.confidence().score, 1.0);
    }

    #[test]
    fn serialization_roundtrip() {
        let inc = CanonicalIncident::new("T".into(), "D".into(), Severity::Critical);
        let json = serde_json::to_string(&inc).unwrap();
        let deserialized: CanonicalIncident = serde_json::from_str(&json).unwrap();
        assert_eq!(inc.id, deserialized.id);
        assert_eq!(inc.title, deserialized.title);
    }
}
