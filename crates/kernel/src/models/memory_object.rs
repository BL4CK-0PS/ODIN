use crate::{
    Confidence, ConfidenceSource, IntelligenceObject, KernelError, Provenance, SourceType,
};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_memory_has_uuid() {
        let mem = MemoryObject::new(
            "inc-1".into(),
            "Phishing campaign".into(),
            serde_json::json!({"severity": "High"}),
            0.85,
        );
        assert!(!mem.id.is_empty());
        assert_eq!(mem.incident_id, "inc-1");
        assert_eq!(mem.summary, "Phishing campaign");
        assert_eq!(mem.confidence, 0.85);
        assert_eq!(mem.version, 1);
    }

    #[test]
    fn confidence_is_clamped() {
        let mem = MemoryObject::new("i".into(), "s".into(), serde_json::json!({}), 1.5);
        assert_eq!(mem.confidence, 1.0);
        let mem2 = MemoryObject::new("i".into(), "s".into(), serde_json::json!({}), -0.5);
        assert_eq!(mem2.confidence, 0.0);
    }

    #[test]
    fn validate_rejects_empty_summary() {
        let mem = MemoryObject::new("i".into(), "".into(), serde_json::json!({}), 0.5);
        assert!(mem.validate().is_err());
    }

    #[test]
    fn validate_rejects_invalid_confidence() {
        let mut mem = MemoryObject::new("i".into(), "s".into(), serde_json::json!({}), 0.5);
        mem.confidence = 2.0;
        assert!(mem.validate().is_err());
    }

    #[test]
    fn validate_accepts_valid_memory() {
        let mem = MemoryObject::new("i".into(), "s".into(), serde_json::json!({}), 0.7);
        assert!(mem.validate().is_ok());
    }

    #[test]
    fn serialization_roundtrip() {
        let mem = MemoryObject::new("i".into(), "s".into(), serde_json::json!({"k": "v"}), 0.9);
        let json = serde_json::to_string(&mem).unwrap();
        let deserialized: MemoryObject = serde_json::from_str(&json).unwrap();
        assert_eq!(mem.id, deserialized.id);
        assert_eq!(mem.summary, deserialized.summary);
    }
}
