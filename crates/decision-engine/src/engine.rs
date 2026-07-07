use crate::recommendation::{Decision, Recommendation, RecommendationKind};
use odin_kernel::{Confidence, ConfidenceSource, Evidence, EvidenceType, IntelligenceObject, KernelError};

pub struct DecisionEngine;

impl DecisionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, evidence: &[Evidence]) -> Result<Decision, KernelError> {
        for e in evidence {
            e.validate()?;
        }
        let recommendations = self.generate_recommendations(evidence);
        let overall_confidence = self.compute_overall_confidence(&recommendations);
        let supporting_evidence: Vec<String> = evidence.iter().map(|e| e.id.clone()).collect();
        Ok(Decision {
            recommendations,
            overall_confidence,
            supporting_evidence,
        })
    }

    fn generate_recommendations(&self, evidence: &[Evidence]) -> Vec<Recommendation> {
        let mut recs = Vec::new();
        if !evidence.is_empty() {
            let sources: Vec<ConfidenceSource> = evidence
                .iter()
                .map(|e| ConfidenceSource {
                    label: format!("evidence:{}", e.id),
                    trust: e.trust_score,
                })
                .collect();
            let evidence_ids: Vec<String> = evidence.iter().map(|e| e.id.clone()).collect();
            recs.push(Recommendation::new(
                RecommendationKind::Investigate,
                "Investigate evidence".into(),
                format!("Review {} pieces of evidence for indicators of compromise", evidence.len()),
                evidence_ids.clone(),
                sources.clone(),
            ));
            let has_network = evidence.iter().any(|e| {
                matches!(e.content_type, EvidenceType::NetworkCapture)
            });
            if has_network {
                recs.push(Recommendation::new(
                    RecommendationKind::Contain,
                    "Contain network activity".into(),
                    "Network-level indicators detected; isolate affected systems".into(),
                    evidence_ids.clone(),
                    sources.clone(),
                ));
            }
        }
        recs
    }

    fn compute_overall_confidence(&self, recommendations: &[Recommendation]) -> Confidence {
        let sources: Vec<ConfidenceSource> = recommendations
            .iter()
            .flat_map(|r| r.confidence.sources.clone())
            .collect();
        Confidence::new(sources)
    }
}

impl Default for DecisionEngine {
    fn default() -> Self {
        Self::new()
    }
}
