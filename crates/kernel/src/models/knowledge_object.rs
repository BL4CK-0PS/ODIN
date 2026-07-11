use crate::{Confidence, ConfidenceSource, IntelligenceObject, KernelError, Provenance, SourceType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KnowledgeStatus {
    Draft,
    Review,
    Active,
    Deprecated,
    Archived,
    Purged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeObject {
    pub id: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub object_type: KnowledgeType,
    pub status: KnowledgeStatus,
    pub tags: Vec<String>,
    pub source_incidents: Vec<String>,
    pub mitre_techniques: Vec<String>,
    pub confidence_sources: Vec<ConfidenceSource>,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub status_history: Vec<StatusTransition>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub review_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KnowledgeType {
    Playbook,
    ThreatIntel,
    MitreMapping,
    IocDefinition,
    ResponseProcedure,
    Policy,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusTransition {
    pub from: KnowledgeStatus,
    pub to: KnowledgeStatus,
    pub reason: String,
    pub actor: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl KnowledgeObject {
    pub fn new(
        title: String,
        description: String,
        content: String,
        object_type: KnowledgeType,
        created_by: String,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            content,
            object_type,
            status: KnowledgeStatus::Draft,
            tags: Vec::new(),
            source_incidents: Vec::new(),
            mitre_techniques: Vec::new(),
            confidence_sources: Vec::new(),
            created_by: created_by.clone(),
            updated_by: created_by,
            created_at: now,
            updated_at: now,
            status_history: Vec::new(),
            expires_at: None,
            review_notes: None,
        }
    }

    pub fn transition(
        &mut self,
        new_status: KnowledgeStatus,
        reason: &str,
        actor: &str,
    ) -> Result<(), KernelError> {
        if !self.can_transition(&new_status) {
            return Err(KernelError::Conflict(format!(
                "Cannot transition from {:?} to {:?}",
                self.status, new_status
            )));
        }

        let transition = StatusTransition {
            from: self.status.clone(),
            to: new_status.clone(),
            reason: reason.to_string(),
            actor: actor.to_string(),
            timestamp: chrono::Utc::now(),
        };

        self.status_history.push(transition);
        self.status = new_status;
        self.updated_by = actor.to_string();
        self.updated_at = chrono::Utc::now();

        Ok(())
    }

    pub fn can_transition(&self, new_status: &KnowledgeStatus) -> bool {
        matches!(
            (&self.status, new_status),
            (KnowledgeStatus::Draft, KnowledgeStatus::Review)
                | (KnowledgeStatus::Review, KnowledgeStatus::Active)
                | (KnowledgeStatus::Review, KnowledgeStatus::Draft)
                | (KnowledgeStatus::Active, KnowledgeStatus::Deprecated)
                | (KnowledgeStatus::Active, KnowledgeStatus::Archived)
                | (KnowledgeStatus::Deprecated, KnowledgeStatus::Active)
                | (KnowledgeStatus::Deprecated, KnowledgeStatus::Archived)
                | (KnowledgeStatus::Archived, KnowledgeStatus::Purged)
        )
    }

    pub fn available_transitions(&self) -> Vec<KnowledgeStatus> {
        match &self.status {
            KnowledgeStatus::Draft => vec![KnowledgeStatus::Review],
            KnowledgeStatus::Review => vec![KnowledgeStatus::Active, KnowledgeStatus::Draft],
            KnowledgeStatus::Active => vec![KnowledgeStatus::Deprecated, KnowledgeStatus::Archived],
            KnowledgeStatus::Deprecated => vec![KnowledgeStatus::Active, KnowledgeStatus::Archived],
            KnowledgeStatus::Archived => vec![KnowledgeStatus::Purged],
            KnowledgeStatus::Purged => vec![],
        }
    }

    pub fn is_editable(&self) -> bool {
        matches!(self.status, KnowledgeStatus::Draft | KnowledgeStatus::Review)
    }

    pub fn is_active(&self) -> bool {
        self.status == KnowledgeStatus::Active
    }
}

impl IntelligenceObject for KnowledgeObject {
    fn id(&self) -> &str {
        &self.id
    }

    fn object_type(&self) -> &'static str {
        "KnowledgeObject"
    }

    fn validate(&self) -> Result<(), KernelError> {
        if self.title.is_empty() {
            return Err(KernelError::Validation("Title cannot be empty".into()));
        }
        if self.content.is_empty() {
            return Err(KernelError::Validation("Content cannot be empty".into()));
        }
        Ok(())
    }

    fn confidence(&self) -> Confidence {
        Confidence::new(self.confidence_sources.clone())
    }

    fn provenance(&self) -> Provenance {
        Provenance {
            source: "knowledge_store".to_string(),
            source_type: SourceType::Other("knowledge".to_string()),
            collected_by: self.created_by.clone(),
            collected_at: self.created_at,
            confidence: self.confidence().score,
        }
    }
}
