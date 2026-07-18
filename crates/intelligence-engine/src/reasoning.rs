use odin_kernel::{Confidence, ConfidenceSource};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningResult {
    pub hypothesis: String,
    pub confidence: Confidence,
    pub supporting_evidence_ids: Vec<String>,
    pub matched_rules: Vec<String>,
    pub reasoning_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningRule {
    pub name: String,
    pub description: String,
    pub weight: f64,
}

pub struct ReasoningEngine {
    rules: Vec<ReasoningRule>,
}

impl ReasoningEngine {
    pub fn new() -> Self {
        Self {
            rules: vec![
                ReasoningRule {
                    name: "known_ioc_match".into(),
                    description: "Evidence matches known IOC pattern".into(),
                    weight: 0.9,
                },
                ReasoningRule {
                    name: "mitre_technique_match".into(),
                    description: "Evidence maps to known MITRE ATT&CK technique".into(),
                    weight: 0.85,
                },
                ReasoningRule {
                    name: "entity_correlation".into(),
                    description: "Multiple entities share common relationship".into(),
                    weight: 0.75,
                },
                ReasoningRule {
                    name: "temporal_proximity".into(),
                    description: "Events occurred within relevant time window".into(),
                    weight: 0.7,
                },
            ],
        }
    }

    pub fn evaluate(&self, evidence_trust_scores: &[(String, f64)]) -> Vec<ReasoningResult> {
        if evidence_trust_scores.is_empty() {
            return Vec::new();
        }
        let avg_trust: f64 = evidence_trust_scores.iter().map(|(_, t)| t).sum::<f64>()
            / evidence_trust_scores.len() as f64;
        self.rules
            .iter()
            .map(|rule| {
                let rule_confidence = avg_trust * rule.weight;
                let sources: Vec<ConfidenceSource> = evidence_trust_scores
                    .iter()
                    .map(|(id, trust)| ConfidenceSource {
                        label: format!("evidence:{}", id),
                        trust: *trust,
                    })
                    .collect();
                ReasoningResult {
                    hypothesis: format!(
                        "Rule '{}': {} (confidence: {:.2})",
                        rule.name, rule.description, rule_confidence
                    ),
                    confidence: Confidence::new(sources),
                    supporting_evidence_ids: evidence_trust_scores
                        .iter()
                        .map(|(id, _)| id.clone())
                        .collect(),
                    matched_rules: vec![rule.name.clone()],
                    reasoning_steps: Vec::new(),
                }
            })
            .collect()
    }
}

impl Default for ReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_empty_returns_empty() {
        let engine = ReasoningEngine::new();
        assert!(engine.evaluate(&[]).is_empty());
    }

    #[test]
    fn evaluate_returns_results_per_rule() {
        let engine = ReasoningEngine::new();
        let scores = vec![("e1".into(), 0.9)];
        let results = engine.evaluate(&scores);
        assert_eq!(results.len(), 4);
    }

    #[test]
    fn evaluate_confidence_uses_geometric_mean_of_trust() {
        let engine = ReasoningEngine::new();
        let scores = vec![("e1".into(), 0.8)];
        let results = engine.evaluate(&scores);
        let first = &results[0];
        assert!((first.confidence.score - 0.8).abs() < 0.01);
    }

    #[test]
    fn evaluate_supports_multiple_evidence() {
        let engine = ReasoningEngine::new();
        let scores = vec![("e1".into(), 0.9), ("e2".into(), 0.7)];
        let results = engine.evaluate(&scores);
        for r in &results {
            assert_eq!(r.supporting_evidence_ids.len(), 2);
        }
    }

    #[test]
    fn evaluate_matched_rules_contains_rule_name() {
        let engine = ReasoningEngine::new();
        let scores = vec![("e1".into(), 1.0)];
        let results = engine.evaluate(&scores);
        let expected = vec![
            "known_ioc_match",
            "mitre_technique_match",
            "entity_correlation",
            "temporal_proximity",
        ];
        for (r, name) in results.iter().zip(expected) {
            assert_eq!(r.matched_rules[0], name);
        }
    }

    #[test]
    fn default_has_four_rules() {
        let engine = ReasoningEngine::default();
        let scores = vec![("a".into(), 0.5)];
        assert_eq!(engine.evaluate(&scores).len(), 4);
    }
}
