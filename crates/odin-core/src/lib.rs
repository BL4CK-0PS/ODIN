pub use odin_kernel;
pub use odin_intelligence_engine;
pub use odin_memory_engine;
pub use odin_retrieval_engine;
pub use odin_decision_engine;
pub use odin_policy_gate;
pub use odin_infrastructure;

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
