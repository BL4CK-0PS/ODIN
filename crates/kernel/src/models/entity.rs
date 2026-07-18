use crate::{
    Confidence, ConfidenceSource, IntelligenceObject, KernelError, Provenance, SourceType,
};
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
        Provenance::new(
            self.name.clone(),
            SourceType::Other("entity".into()),
            "system".into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_entity_has_uuid() {
        let ent = Entity::new("evil.com".into(), EntityType::Domain, serde_json::json!({}));
        assert!(!ent.id.is_empty());
        assert_eq!(ent.name, "evil.com");
        assert!(matches!(ent.entity_type, EntityType::Domain));
    }

    #[test]
    fn validate_rejects_empty_name() {
        let ent = Entity::new("".into(), EntityType::IpAddress, serde_json::json!({}));
        assert!(ent.validate().is_err());
    }

    #[test]
    fn validate_rejects_whitespace_name() {
        let ent = Entity::new("   ".into(), EntityType::File, serde_json::json!({}));
        assert!(ent.validate().is_err());
    }

    #[test]
    fn validate_accepts_valid_entity() {
        let ent = Entity::new(
            "10.0.0.1".into(),
            EntityType::IpAddress,
            serde_json::json!({}),
        );
        assert!(ent.validate().is_ok());
    }

    #[test]
    fn entity_confidence_is_0_97() {
        let ent = Entity::new("test".into(), EntityType::Process, serde_json::json!({}));
        assert_eq!(ent.confidence().score, 0.97);
    }

    #[test]
    fn serialization_roundtrip() {
        let ent = Entity::new(
            "test.exe".into(),
            EntityType::File,
            serde_json::json!({"path": "/tmp"}),
        );
        let json = serde_json::to_string(&ent).unwrap();
        let deserialized: Entity = serde_json::from_str(&json).unwrap();
        assert_eq!(ent.id, deserialized.id);
        assert_eq!(ent.name, deserialized.name);
    }
}
