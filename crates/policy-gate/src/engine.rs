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
        self.policies
            .iter()
            .map(|p| p.evaluate(confidence))
            .collect()
    }

    pub fn is_allowed(&self, confidence: f64) -> bool {
        self.enforce(confidence)
            .iter()
            .all(|r| matches!(r.verdict, PolicyVerdict::Allow))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_gate_has_two_policies() {
        let gate = PolicyGate::new();
        assert_eq!(gate.enforce(0.5).len(), 2);
    }

    #[test]
    fn high_confidence_is_allowed() {
        let gate = PolicyGate::new();
        assert!(gate.is_allowed(0.85));
    }

    #[test]
    fn low_confidence_is_denied() {
        let gate = PolicyGate::new();
        assert!(!gate.is_allowed(0.5));
    }

    #[test]
    fn denied_reasons_list_contains_min_confidence() {
        let gate = PolicyGate::new();
        let reasons = gate.denied_reasons(0.5);
        assert!(!reasons.is_empty());
        assert!(reasons.iter().any(|r| r.contains("min_confidence")));
    }

    #[test]
    fn custom_policy_rejects_based_on_threshold() {
        let policy = Policy::new("strict".into(), "Must be above 0.9".into(), 0.9);
        let gate = PolicyGate::with_policies(vec![policy]);
        assert!(!gate.is_allowed(0.85));
        let reasons = gate.denied_reasons(0.85);
        assert_eq!(reasons.len(), 1);
        assert!(reasons[0].contains("strict"));
    }

    #[test]
    fn multiple_policies_all_must_pass() {
        let policies = vec![
            Policy::new("p1".into(), "Above 0.6".into(), 0.6),
            Policy::new("p2".into(), "Above 0.8".into(), 0.8),
        ];
        let gate = PolicyGate::with_policies(policies);
        assert!(!gate.is_allowed(0.7));
        let reasons = gate.denied_reasons(0.7);
        assert_eq!(reasons.len(), 1);
        assert!(reasons[0].contains("p2"));
    }

    #[test]
    fn empty_gate_allows_everything() {
        let gate = PolicyGate::with_policies(vec![]);
        assert!(gate.is_allowed(0.0));
    }

    #[test]
    fn exactly_at_threshold_is_allowed() {
        let gate = PolicyGate::new();
        assert!(gate.is_allowed(0.70));
    }
}
