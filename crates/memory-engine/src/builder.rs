use odin_kernel::{
    CanonicalIncident, Confidence, ConfidenceSource, IntelligenceObject, KernelError, MemoryObject,
};

pub struct MemoryBuilder;

impl MemoryBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self, incident: &CanonicalIncident) -> Result<MemoryObject, KernelError> {
        incident.validate()?;
        let confidence = self.compute_confidence(incident);
        let summary = self.generate_summary(incident);
        let context = self.build_context(incident);
        let mut memory = MemoryObject::new(incident.id.clone(), summary, context, confidence.score);
        memory.version = 1;
        Ok(memory)
    }

    fn compute_confidence(&self, incident: &CanonicalIncident) -> Confidence {
        let sources = vec![ConfidenceSource {
            label: format!("incident:{}", incident.id),
            trust: 1.0,
        }];
        Confidence::new(sources)
    }

    fn generate_summary(&self, incident: &CanonicalIncident) -> String {
        format!(
            "Incident: {} — {}. Severity: {:?}. Techniques: {}.",
            incident.title,
            incident.description,
            incident.severity,
            incident.mitre_techniques.join(", "),
        )
    }

    fn build_context(&self, incident: &CanonicalIncident) -> serde_json::Value {
        serde_json::json!({
            "title": incident.title,
            "severity": format!("{:?}", incident.severity),
            "status": format!("{:?}", incident.status),
            "techniques": incident.mitre_techniques,
            "tags": incident.tags,
            "evidence_count": incident.evidence_ids.len(),
            "entity_count": incident.entity_ids.len(),
        })
    }
}

impl Default for MemoryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
