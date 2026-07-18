use crate::recommendation::{Decision, Recommendation, RecommendationKind};
use odin_kernel::{
    Confidence, ConfidenceSource, Evidence, EvidenceType, IntelligenceObject, KernelError,
};

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
                format!(
                    "Review {} pieces of evidence for indicators of compromise",
                    evidence.len()
                ),
                evidence_ids.clone(),
                sources.clone(),
            ));
            let has_network = evidence
                .iter()
                .any(|e| matches!(e.content_type, EvidenceType::NetworkCapture));
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

#[cfg(test)]
mod tests {
    use super::*;
    use odin_kernel::{Evidence, EvidenceType};

    fn make_evidence(
        source: &str,
        content: &str,
        content_type: EvidenceType,
        trust: f64,
    ) -> Evidence {
        Evidence::new(
            "inc-1".into(),
            source.into(),
            content.into(),
            content_type,
            trust,
        )
    }

    #[test]
    fn evaluate_empty_evidence() {
        let engine = DecisionEngine::new();
        let decision = engine.evaluate(&[]).unwrap();
        assert!(decision.recommendations.is_empty());
        assert!(decision.supporting_evidence.is_empty());
    }

    #[test]
    fn evaluate_single_log_evidence() {
        let engine = DecisionEngine::new();
        let ev = make_evidence("firewall.log", "blocked connection", EvidenceType::Log, 0.9);
        let decision = engine.evaluate(&[ev.clone()]).unwrap();
        assert_eq!(decision.recommendations.len(), 1);
        assert!(matches!(
            decision.recommendations[0].kind,
            RecommendationKind::Investigate
        ));
        assert_eq!(decision.supporting_evidence.len(), 1);
        assert_eq!(decision.supporting_evidence[0], ev.id);
    }

    #[test]
    fn evaluate_network_evidence_adds_contain() {
        let engine = DecisionEngine::new();
        let ev = make_evidence(
            "pcap",
            "suspicious traffic",
            EvidenceType::NetworkCapture,
            0.85,
        );
        let decision = engine.evaluate(&[ev]).unwrap();
        assert_eq!(decision.recommendations.len(), 2);
        let kinds: Vec<_> = decision
            .recommendations
            .iter()
            .map(|r| format!("{:?}", r.kind))
            .collect();
        assert!(kinds.contains(&"Investigate".to_string()));
        assert!(kinds.contains(&"Contain".to_string()));
    }

    #[test]
    fn evaluate_multiple_evidence() {
        let engine = DecisionEngine::new();
        let ev1 = make_evidence("log1", "auth failure", EvidenceType::Log, 0.9);
        let ev2 = make_evidence("net1", "c2 traffic", EvidenceType::NetworkCapture, 0.8);
        let decision = engine.evaluate(&[ev1, ev2]).unwrap();
        assert!(decision.recommendations.len() >= 2);
        assert!(!decision.supporting_evidence.is_empty());
    }

    #[test]
    fn overall_confidence_is_computed() {
        let engine = DecisionEngine::new();
        let ev = make_evidence("s", "c", EvidenceType::Log, 0.9);
        let decision = engine.evaluate(&[ev]).unwrap();
        assert!(decision.overall_confidence.score > 0.0);
    }

    #[test]
    fn evaluate_rejects_invalid_evidence() {
        let engine = DecisionEngine::new();
        let ev = Evidence::new("i".into(), "".into(), "c".into(), EvidenceType::Log, 0.5);
        assert!(engine.evaluate(&[ev]).is_err());
    }
}
