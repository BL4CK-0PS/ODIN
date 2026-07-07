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
