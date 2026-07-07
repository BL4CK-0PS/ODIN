use crate::policy::{Policy, PolicyResult, PolicyVerdict};

pub struct PolicyGate {
    policies: Vec<Policy>,
}

impl PolicyGate {
    pub fn new() -> Self {
        Self {
            policies: vec![
                Policy::new(
                    "min_confidence".into(),
                    "Never recommend with confidence < 0.70".into(),
                    0.70,
                ),
                Policy::new(
                    "no_restricted_evidence".into(),
                    "Never expose restricted evidence".into(),
                    0.0,
                ),
            ],
        }
    }

    pub fn with_policies(policies: Vec<Policy>) -> Self {
        Self { policies }
    }

    pub fn enforce(&self, confidence: f64) -> Vec<PolicyResult> {
        self.policies.iter().map(|p| p.evaluate(confidence)).collect()
    }

    pub fn is_allowed(&self, confidence: f64) -> bool {
        self.enforce(confidence).iter().all(|r| matches!(r.verdict, PolicyVerdict::Allow))
    }

    pub fn denied_reasons(&self, confidence: f64) -> Vec<String> {
        self.enforce(confidence)
            .into_iter()
            .filter_map(|r| match r.verdict {
                PolicyVerdict::Deny(reason) => Some(format!("{}: {}", r.policy, reason)),
                PolicyVerdict::Allow => None,
            })
            .collect()
    }
}

impl Default for PolicyGate {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for PolicyGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PolicyGate").finish()
    }
}
