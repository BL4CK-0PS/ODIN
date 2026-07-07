use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyVerdict {
    Allow,
    Deny(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResult {
    pub policy: String,
    pub verdict: PolicyVerdict,
}

#[derive(Debug, Clone)]
pub struct Policy {
    pub name: String,
    pub description: String,
    pub min_confidence: f64,
}

impl Policy {
    pub fn new(name: String, description: String, min_confidence: f64) -> Self {
        Self {
            name,
            description,
            min_confidence,
        }
    }

    pub fn evaluate(&self, confidence: f64) -> PolicyResult {
        if confidence >= self.min_confidence {
            PolicyResult {
                policy: self.name.clone(),
                verdict: PolicyVerdict::Allow,
            }
        } else {
            PolicyResult {
                policy: self.name.clone(),
                verdict: PolicyVerdict::Deny(format!(
                    "Confidence {:.2} is below minimum {:.2}",
                    confidence, self.min_confidence
                )),
            }
        }
    }
}
