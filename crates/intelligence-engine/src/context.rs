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

#[cfg(test)]
mod tests {
    use super::*;
    use odin_kernel::ConfidenceSource;

    #[test]
    fn apply_known_factor() {
        let engine = ContextEngine::new();
        let base = Confidence::default();
        let factors = vec![ContextWeight {
            factor: "severity".into(),
            weight: 0.8,
        }];
        let result = engine.apply_context(&base, &factors);
        assert_eq!(result.sources.len(), 1);
        assert!((result.sources[0].trust - 0.8).abs() < 0.01);
    }

    #[test]
    fn apply_unknown_factor_is_ignored() {
        let engine = ContextEngine::new();
        let base = Confidence::default();
        let factors = vec![ContextWeight {
            factor: "unknown".into(),
            weight: 0.5,
        }];
        let result = engine.apply_context(&base, &factors);
        assert!(result.sources.is_empty());
    }

    #[test]
    fn apply_multiple_factors() {
        let engine = ContextEngine::new();
        let base = Confidence::default();
        let factors = vec![
            ContextWeight {
                factor: "severity".into(),
                weight: 0.8,
            },
            ContextWeight {
                factor: "recency".into(),
                weight: 0.9,
            },
        ];
        let result = engine.apply_context(&base, &factors);
        assert_eq!(result.sources.len(), 2);
    }

    #[test]
    fn apply_context_preserves_existing_sources() {
        let engine = ContextEngine::new();
        let base = Confidence::new(vec![ConfidenceSource {
            label: "existing".into(),
            trust: 0.5,
        }]);
        let factors = vec![ContextWeight {
            factor: "severity".into(),
            weight: 0.8,
        }];
        let result = engine.apply_context(&base, &factors);
        assert_eq!(result.sources.len(), 2);
        assert_eq!(result.sources[0].label, "existing");
    }

    #[test]
    fn clamp_prevents_over_one() {
        let engine = ContextEngine::new();
        let base = Confidence::default();
        let factors = vec![ContextWeight {
            factor: "severity".into(),
            weight: 5.0,
        }];
        let result = engine.apply_context(&base, &factors);
        assert!(result.sources[0].trust <= 1.0);
    }
}
