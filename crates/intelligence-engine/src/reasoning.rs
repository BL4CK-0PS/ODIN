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

    pub fn evaluate(
        &self,
        evidence_trust_scores: &[(String, f64)],
    ) -> Vec<ReasoningResult> {
        if evidence_trust_scores.is_empty() {
            return Vec::new();
        }
        let avg_trust: f64 =
            evidence_trust_scores.iter().map(|(_, t)| t).sum::<f64>() / evidence_trust_scores.len() as f64;
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
                    supporting_evidence_ids: evidence_trust_scores.iter().map(|(id, _)| id.clone()).collect(),
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
