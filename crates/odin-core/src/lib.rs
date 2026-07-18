pub use odin_decision_engine;
pub use odin_infrastructure;
pub use odin_intelligence_engine;
pub use odin_kernel;
pub use odin_memory_engine;
pub use odin_policy_gate;
pub use odin_retrieval_engine;

pub struct Odin {
    pub intelligence: odin_intelligence_engine::IntelligenceEngine,
    pub memory: odin_memory_engine::MemoryEngine,
    pub retrieval: odin_retrieval_engine::RetrievalEngine,
    pub decision: odin_decision_engine::DecisionEngine,
    pub policy: odin_policy_gate::PolicyGate,
}

impl Odin {
    pub fn new() -> Self {
        Self {
            intelligence: odin_intelligence_engine::IntelligenceEngine::new(),
            memory: odin_memory_engine::MemoryEngine::new(),
            retrieval: odin_retrieval_engine::RetrievalEngine::new(),
            decision: odin_decision_engine::DecisionEngine::new(),
            policy: odin_policy_gate::PolicyGate::new(),
        }
    }
}

impl Default for Odin {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use odin_kernel::{Evidence, EvidenceType};

    #[test]
    fn odin_new_creates_all_engines() {
        let odin = Odin::new();
        assert!(!odin.intelligence.has_ollama());
    }

    #[test]
    fn odin_default_matches_new() {
        let odin = Odin::default();
        assert!(!odin.intelligence.has_ollama());
    }

    #[test]
    fn odin_decision_engine_works() {
        let odin = Odin::new();
        let ev = Evidence::new("i".into(), "s".into(), "c".into(), EvidenceType::Log, 0.9);
        let decision = odin.decision.evaluate(&[ev]).unwrap();
        assert!(!decision.recommendations.is_empty());
    }

    #[test]
    fn odin_policy_gate_allows_high_confidence() {
        let odin = Odin::new();
        assert!(odin.policy.is_allowed(0.85));
    }

    #[test]
    fn odin_policy_gate_denies_low_confidence() {
        let odin = Odin::new();
        assert!(!odin.policy.is_allowed(0.5));
    }

    #[test]
    fn odin_intelligence_pipeline_works() {
        let mut odin = Odin::new();
        let ev = Evidence::new("i".into(), "s".into(), "c".into(), EvidenceType::Log, 0.8);
        let result = odin.intelligence.analyze(&[ev]).unwrap();
        assert!(!result.results.is_empty());
    }
}
