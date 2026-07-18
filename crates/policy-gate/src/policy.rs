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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_allows_high_confidence() {
        let p = Policy::new("test".into(), "desc".into(), 0.7);
        let result = p.evaluate(0.8);
        assert!(matches!(result.verdict, PolicyVerdict::Allow));
    }

    #[test]
    fn policy_denies_low_confidence() {
        let p = Policy::new("test".into(), "desc".into(), 0.7);
        let result = p.evaluate(0.3);
        match result.verdict {
            PolicyVerdict::Deny(msg) => assert!(msg.contains("0.30")),
            _ => panic!("expected Deny"),
        }
    }

    #[test]
    fn policy_exactly_at_threshold_is_allow() {
        let p = Policy::new("test".into(), "desc".into(), 0.7);
        assert!(matches!(p.evaluate(0.7).verdict, PolicyVerdict::Allow));
    }

    #[test]
    fn policy_result_serialization() {
        let p = Policy::new("n".into(), "d".into(), 0.5);
        let result = p.evaluate(0.6);
        let json = serde_json::to_string(&result).unwrap();
        let _: PolicyResult = serde_json::from_str(&json).unwrap();
    }
}
