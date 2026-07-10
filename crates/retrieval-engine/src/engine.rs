use crate::qdrant::QdrantClient;
use crate::ranking::{RankedResult, Ranker};
use crate::similarity::{HybridScore, SemanticScorer, StructuralScorer};
use odin_kernel::{CanonicalIncident, KernelError, MemoryObject};
use std::collections::HashMap;
use std::sync::RwLock;

type EmbedFn = Box<dyn Fn(&str) -> Result<Vec<f64>, KernelError> + Send + Sync>;

pub struct RetrievalEngine {
    structural: StructuralScorer,
    semantic: SemanticScorer,
    qdrant: Option<QdrantClient>,
    embed_fn: Option<EmbedFn>,
    feedback_signals: RwLock<HashMap<String, f64>>,
}

impl RetrievalEngine {
    pub fn new() -> Self {
        Self {
            structural: StructuralScorer::new(),
            semantic: SemanticScorer::new(),
            qdrant: None,
            embed_fn: None,
            feedback_signals: RwLock::new(HashMap::new()),
        }
    }

    pub fn with_qdrant(
        qdrant: QdrantClient,
        embed_fn: EmbedFn,
    ) -> Self {
        Self {
            structural: StructuralScorer::new(),
            semantic: SemanticScorer::new(),
            qdrant: Some(qdrant),
            embed_fn: Some(embed_fn),
            feedback_signals: RwLock::new(HashMap::new()),
        }
    }

    pub fn set_feedback_signal(&self, incident_id: &str, avg_rating: f64) {
        if let Ok(mut signals) = self.feedback_signals.write() {
            let signal = (avg_rating / 5.0).clamp(0.0, 1.0);
            signals.insert(incident_id.to_string(), signal);
        }
    }

    pub fn remove_feedback_signal(&self, incident_id: &str) {
        if let Ok(mut signals) = self.feedback_signals.write() {
            signals.remove(incident_id);
        }
    }

    fn get_feedback_boost(&self, candidate: &MemoryObject) -> f64 {
        let incident_id = &candidate.incident_id;
        let key = &candidate.id;
        if let Ok(signals) = self.feedback_signals.read() {
            signals.get(incident_id).or_else(|| signals.get(key)).copied().unwrap_or(0.5)
        } else {
            0.5
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
                let context = Self::severity_score(query, candidate);
                let feedback = self.get_feedback_boost(candidate);
                let overall = 0.35 * structural + 0.35 * semantic + 0.15 * context + 0.15 * feedback;
                (candidate.clone(), HybridScore { overall, structural, semantic, context })
            })
            .collect();
        let ranked = Ranker::rank(scored);
        Ok(ranked.into_iter().take(top_k).collect())
    }

    pub async fn search_hybrid(
        &self,
        query: &CanonicalIncident,
        candidates: &[MemoryObject],
        query_text: &str,
        top_k: usize,
    ) -> Result<Vec<RankedResult>, KernelError> {
        let mut scored: Vec<(MemoryObject, HybridScore)> = candidates
            .iter()
            .map(|candidate| {
                let structural = self.structural.score(query, candidate);
                let semantic = self.semantic.score(query, candidate);
                let context = Self::severity_score(query, candidate);
                let feedback = self.get_feedback_boost(candidate);
                let overall = 0.35 * structural + 0.35 * semantic + 0.15 * context + 0.15 * feedback;
                (candidate.clone(), HybridScore { overall, structural, semantic, context })
            })
            .collect();

        if let Some(ref qdrant) = self.qdrant {
            if let Some(ref embed_fn) = self.embed_fn {
                match embed_fn(query_text) {
                    Ok(query_vec) => {
                        match qdrant.search(query_vec, top_k * 2).await {
                            Ok(qdrant_results) => {
                                let qdrant_ids: std::collections::HashSet<String> = qdrant_results
                                    .iter()
                                    .map(|r| r.id.clone())
                                    .collect();
                                for (memory, score) in &mut scored {
                                    let boost = if qdrant_ids.contains(&memory.id) {
                                        0.3
                                    } else {
                                        0.0
                                    };
                                    score.structural += boost * 0.5;
                                    score.semantic += boost * 0.5;
                                    score.overall = 0.35 * score.structural + 0.35 * score.semantic + 0.15 * score.context + 0.15 * self.get_feedback_boost(memory);
                                }
                            }
                            Err(e) => {
                                tracing::warn!("Qdrant search failed, falling back to local: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Embedding failed, skipping Qdrant: {}", e);
                    }
                }
            }
        }

        let ranked = Ranker::rank(scored);
        Ok(ranked.into_iter().take(top_k).collect())
    }

    fn severity_score(query: &CanonicalIncident, candidate: &MemoryObject) -> f64 {
        if let Some(sev) = candidate.context.get("severity").and_then(|v| v.as_str()) {
            let query_sev = format!("{:?}", query.severity);
            if sev == query_sev { 1.0 } else { 0.3 }
        } else {
            0.0
        }
    }
}

impl Default for RetrievalEngine {
    fn default() -> Self {
        Self::new()
    }
}
