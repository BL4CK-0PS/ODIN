use odin_kernel::Confidence;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextWeight {
    pub factor: String,
    pub weight: f64,
}

pub struct ContextEngine {
    weights: HashMap<String, f64>,
}

impl ContextEngine {
    pub fn new() -> Self {
        let mut weights = HashMap::new();
        weights.insert("severity".into(), 1.0);
        weights.insert("recency".into(), 0.9);
        weights.insert("entity_overlap".into(), 0.8);
        weights.insert("technique_overlap".into(), 0.85);
        Self { weights }
    }

    pub fn apply_context(&self, confidence: &Confidence, factors: &[ContextWeight]) -> Confidence {
        let mut sources = confidence.sources.clone();
        for factor in factors {
            if let Some(base_weight) = self.weights.get(&factor.factor) {
                let adjusted = factor.weight * base_weight;
                sources.push(odin_kernel::ConfidenceSource {
                    label: format!("context:{}", factor.factor),
                    trust: adjusted.clamp(0.0, 1.0),
                });
            }
        }
        Confidence::new(sources)
    }
}

impl Default for ContextEngine {
    fn default() -> Self {
        Self::new()
    }
}
