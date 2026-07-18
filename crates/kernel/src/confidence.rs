use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Confidence {
    pub score: f64,
    pub sources: Vec<ConfidenceSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceSource {
    pub label: String,
    pub trust: f64,
}

impl Confidence {
    pub fn new(sources: Vec<ConfidenceSource>) -> Self {
        let score = Self::geometric_mean(&sources);
        Self { score, sources }
    }

    pub fn geometric_mean(sources: &[ConfidenceSource]) -> f64 {
        if sources.is_empty() {
            return 0.0;
        }
        let product: f64 = sources.iter().map(|s| s.trust.max(0.0001)).product();
        product.powf(1.0 / sources.len() as f64)
    }

    pub fn propagate(&self, additional: &[ConfidenceSource]) -> Self {
        let mut combined = self.sources.clone();
        combined.extend_from_slice(additional);
        Self::new(combined)
    }
}

impl Default for Confidence {
    fn default() -> Self {
        Self {
            score: 0.0,
            sources: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometric_mean_empty_is_zero() {
        assert_eq!(Confidence::geometric_mean(&[]), 0.0);
    }

    #[test]
    fn geometric_mean_single_source() {
        let sources = vec![ConfidenceSource {
            label: "a".into(),
            trust: 0.8,
        }];
        let result = Confidence::geometric_mean(&sources);
        assert!((result - 0.8).abs() < 0.001);
    }

    #[test]
    fn geometric_mean_two_sources() {
        let sources = vec![
            ConfidenceSource {
                label: "a".into(),
                trust: 0.9,
            },
            ConfidenceSource {
                label: "b".into(),
                trust: 0.81,
            },
        ];
        let result = Confidence::geometric_mean(&sources);
        // sqrt(0.9 * 0.81) = sqrt(0.729) = 0.854
        assert!((result - 0.854).abs() < 0.01);
    }

    #[test]
    fn propagate_adds_sources() {
        let c1 = Confidence::new(vec![ConfidenceSource {
            label: "a".into(),
            trust: 0.9,
        }]);
        let c2 = c1.propagate(&[ConfidenceSource {
            label: "b".into(),
            trust: 0.8,
        }]);
        assert_eq!(c2.sources.len(), 2);
        assert!(c2.score > 0.0 && c2.score < 1.0);
    }

    #[test]
    fn default_confidence_is_zero() {
        let c = Confidence::default();
        assert_eq!(c.score, 0.0);
        assert!(c.sources.is_empty());
    }
}
