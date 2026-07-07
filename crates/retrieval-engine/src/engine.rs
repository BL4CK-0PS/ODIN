use crate::ranking::{RankedResult, Ranker};
use crate::similarity::{HybridScore, SemanticScorer, StructuralScorer};
use odin_kernel::{CanonicalIncident, KernelError, MemoryObject};

pub struct RetrievalEngine {
    structural: StructuralScorer,
    semantic: SemanticScorer,
}

impl RetrievalEngine {
    pub fn new() -> Self {
        Self {
            structural: StructuralScorer::new(),
            semantic: SemanticScorer::new(),
        }
    }

    pub fn search(
        &self,
        query: &CanonicalIncident,
        candidates: &[MemoryObject],
        top_k: usize,
    ) -> Result<Vec<RankedResult>, KernelError> {
        let scored: Vec<(MemoryObject, HybridScore)> = candidates
            .iter()
            .map(|candidate| {
                let structural = self.structural.score(query, candidate);
                let semantic = self.semantic.score(query, candidate);
                let context = Self::context_score(query, candidate);
                let overall = 0.4 * structural + 0.4 * semantic + 0.2 * context;
                (candidate.clone(), HybridScore { overall, structural, semantic, context })
            })
            .collect();
        let ranked = Ranker::rank(scored);
        Ok(ranked.into_iter().take(top_k).collect())
    }

    fn context_score(query: &CanonicalIncident, candidate: &MemoryObject) -> f64 {
        let severity_match = if let Some(sev) = candidate.context.get("severity").and_then(|v| v.as_str()) {
            let query_sev = format!("{:?}", query.severity);
            if sev == query_sev { 1.0 } else { 0.3 }
        } else {
            0.0
        };
        severity_match
    }
}

impl Default for RetrievalEngine {
    fn default() -> Self {
        Self::new()
    }
}
