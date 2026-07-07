use odin_core::odin_kernel::{CanonicalIncident, Evidence, Entity};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use odin_core::odin_memory_engine::MemoryEngine;
use odin_core::odin_intelligence_engine::IntelligenceEngine;
use odin_core::odin_retrieval_engine::RetrievalEngine;
use odin_core::odin_decision_engine::DecisionEngine;
use odin_core::odin_policy_gate::PolicyGate;

pub type IncidentMap = Arc<RwLock<HashMap<String, CanonicalIncident>>>;
pub type EvidenceMap = Arc<RwLock<HashMap<String, Vec<Evidence>>>>;
pub type EntityMap = Arc<RwLock<HashMap<String, Vec<Entity>>>>;

pub struct AppState {
    pub incidents: IncidentMap,
    pub evidence: EvidenceMap,
    pub entities: EntityMap,
    pub memory: MemoryEngine,
    pub intelligence: Arc<RwLock<IntelligenceEngine>>,
    pub retrieval: RetrievalEngine,
    pub decision: DecisionEngine,
    pub policy: PolicyGate,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            incidents: Arc::new(RwLock::new(HashMap::new())),
            evidence: Arc::new(RwLock::new(HashMap::new())),
            entities: Arc::new(RwLock::new(HashMap::new())),
            memory: MemoryEngine::new(),
            intelligence: Arc::new(RwLock::new(IntelligenceEngine::new())),
            retrieval: RetrievalEngine::new(),
            decision: DecisionEngine::new(),
            policy: PolicyGate::new(),
        }
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}
