use crate::qdrant::QdrantClient;
use crate::ranking::{RankedResult, Ranker};
use crate::rl_feedback::RLFeedbackLoop;
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
    pub rl: RwLock<RLFeedbackLoop>,
}

impl RetrievalEngine {
    pub fn new() -> Self {
        Self {
            structural: StructuralScorer::new(),
            semantic: SemanticScorer::new(),
            qdrant: None,
            embed_fn: None,
            feedback_signals: RwLock::new(HashMap::new()),
            rl: RwLock::new(RLFeedbackLoop::new()),
        }
    }

    pub fn with_qdrant(qdrant: QdrantClient, embed_fn: EmbedFn) -> Self {
        Self {
            structural: StructuralScorer::new(),
            semantic: SemanticScorer::new(),
            qdrant: Some(qdrant),
            embed_fn: Some(embed_fn),
            feedback_signals: RwLock::new(HashMap::new()),
            rl: RwLock::new(RLFeedbackLoop::new()),
        }
    }

    pub fn set_feedback_signal(&self, incident_id: &str, avg_rating: f64) {
        if let Ok(mut signals) = self.feedback_signals.write() {
            let signal = (avg_rating / 5.0).clamp(0.0, 1.0);
            signals.insert(incident_id.to_string(), signal);
        }

        if let Ok(mut rl) = self.rl.write() {
            let reward = (avg_rating / 5.0).clamp(0.0, 1.0) * 5.0;
            rl.record_experience(
                incident_id,
                incident_id,
                "user_feedback",
                reward,
                vec![avg_rating / 5.0],
            );
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
            signals
                .get(incident_id)
                .or_else(|| signals.get(key))
                .copied()
                .unwrap_or(0.5)
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
        let (w_structural, w_semantic, w_context, w_feedback) = self.get_adaptive_weights();

        let scored: Vec<(MemoryObject, HybridScore)> = candidates
            .iter()
            .map(|candidate| {
                let structural = self.structural.score(query, candidate);
                let semantic = self.semantic.score(query, candidate);
                let context = Self::severity_score(query, candidate);
                let feedback = self.get_feedback_boost(candidate);
                let overall = w_structural * structural
                    + w_semantic * semantic
                    + w_context * context
                    + w_feedback * feedback;

                self.record_scoring_experience(query, candidate, overall);

                (
                    candidate.clone(),
                    HybridScore {
                        overall,
                        structural,
                        semantic,
                        context,
                    },
                )
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
        let (w_structural, w_semantic, w_context, w_feedback) = self.get_adaptive_weights();

        let mut scored: Vec<(MemoryObject, HybridScore)> = candidates
            .iter()
            .map(|candidate| {
                let structural = self.structural.score(query, candidate);
                let semantic = self.semantic.score(query, candidate);
                let context = Self::severity_score(query, candidate);
                let feedback = self.get_feedback_boost(candidate);
                let overall = w_structural * structural
                    + w_semantic * semantic
                    + w_context * context
                    + w_feedback * feedback;
                (
                    candidate.clone(),
                    HybridScore {
                        overall,
                        structural,
                        semantic,
                        context,
                    },
                )
            })
            .collect();

        if let Some(ref qdrant) = self.qdrant {
            if let Some(ref embed_fn) = self.embed_fn {
                match embed_fn(query_text) {
                    Ok(query_vec) => match qdrant.search(query_vec, top_k * 2).await {
                        Ok(qdrant_results) => {
                            let qdrant_ids: std::collections::HashSet<String> =
                                qdrant_results.iter().map(|r| r.id.clone()).collect();
                            for (memory, score) in &mut scored {
                                let boost = if qdrant_ids.contains(&memory.id) {
                                    0.3
                                } else {
                                    0.0
                                };
                                score.structural += boost * 0.5;
                                score.semantic += boost * 0.5;
                                let feedback = self.get_feedback_boost(memory);
                                score.overall = w_structural * score.structural
                                    + w_semantic * score.semantic
                                    + w_context * score.context
                                    + w_feedback * feedback;
                            }
                        }
                        Err(e) => {
                            tracing::warn!("Qdrant search failed, falling back to local: {}", e);
                        }
                    },
                    Err(e) => {
                        tracing::warn!("Embedding failed, skipping Qdrant: {}", e);
                    }
                }
            }
        }

        let ranked = Ranker::rank(scored);
        Ok(ranked.into_iter().take(top_k).collect())
    }

    fn get_adaptive_weights(&self) -> (f64, f64, f64, f64) {
        if let Ok(rl) = self.rl.read() {
            if rl.total_updates > 10 {
                return rl.compute_adaptive_weights(0.5, 0.5, 0.5);
            }
        }
        (0.35, 0.35, 0.15, 0.15)
    }

    fn record_scoring_experience(
        &self,
        query: &CanonicalIncident,
        candidate: &MemoryObject,
        score: f64,
    ) {
        if let Ok(mut rl) = self.rl.write() {
            let reward = score * 5.0;
            let features = vec![
                candidate.confidence,
                score,
                match query.severity {
                    odin_kernel::Severity::Critical => 1.0,
                    odin_kernel::Severity::High => 0.8,
                    odin_kernel::Severity::Medium => 0.5,
                    odin_kernel::Severity::Low => 0.3,
                    odin_kernel::Severity::Informational => 0.1,
                },
            ];
            rl.record_experience(&query.id, &candidate.id, "search_rank", reward, features);
        }
    }

    fn severity_score(query: &CanonicalIncident, candidate: &MemoryObject) -> f64 {
        if let Some(sev) = candidate.context.get("severity").and_then(|v| v.as_str()) {
            let query_sev = format!("{:?}", query.severity);
            if sev == query_sev {
                1.0
            } else {
                0.3
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use odin_kernel::{MemoryObject, Severity};

    fn make_query(title: &str, techniques: Vec<&str>, tags: Vec<&str>) -> CanonicalIncident {
        let mut inc =
            CanonicalIncident::new(title.into(), "Test description".into(), Severity::High);
        inc.mitre_techniques = techniques.into_iter().map(String::from).collect();
        inc.tags = tags.into_iter().map(String::from).collect();
        inc
    }

    fn make_memory(
        incident_id: &str,
        summary: &str,
        techniques: Vec<&str>,
        tags: Vec<&str>,
    ) -> MemoryObject {
        let mem = MemoryObject::new(
            incident_id.into(),
            summary.into(),
            serde_json::json!({
                "techniques": techniques,
                "tags": tags,
                "severity": "High",
            }),
            0.9,
        );
        mem
    }

    #[test]
    fn search_returns_ranked_results() {
        let engine = RetrievalEngine::new();
        let query = make_query("Phishing", vec!["T1566"], vec!["phishing"]);
        let candidates = vec![
            make_memory(
                "inc-1",
                "Phishing campaign via email",
                vec!["T1566"],
                vec!["phishing"],
            ),
            make_memory(
                "inc-2",
                "Ransomware deployment",
                vec!["T1486"],
                vec!["ransomware"],
            ),
        ];

        let results = engine.search(&query, &candidates, 10).unwrap();
        assert_eq!(results.len(), 2);
        assert!(results[0].score.overall >= results[1].score.overall);
    }

    #[test]
    fn search_respects_top_k() {
        let engine = RetrievalEngine::new();
        let query = make_query("Test", vec!["T1059"], vec!["test"]);
        let candidates: Vec<MemoryObject> = (0..5)
            .map(|i| {
                make_memory(
                    &format!("inc-{}", i),
                    "Test summary",
                    vec!["T1059"],
                    vec!["test"],
                )
            })
            .collect();

        let results = engine.search(&query, &candidates, 2).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn search_with_empty_candidates() {
        let engine = RetrievalEngine::new();
        let query = make_query("Test", vec![], vec![]);
        let results = engine.search(&query, &[], 10).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn feedback_signal_affects_ranking() {
        let engine = RetrievalEngine::new();
        let query = make_query("Test", vec!["T1059"], vec!["test"]);
        let candidates = vec![
            make_memory("inc-1", "A", vec!["T1059"], vec!["test"]),
            make_memory("inc-2", "B", vec!["T1059"], vec!["test"]),
        ];

        // Without feedback
        let _results_before = engine.search(&query, &candidates, 10).unwrap();

        // Set feedback for second candidate to boost it
        engine.set_feedback_signal("inc-2", 5.0);
        let results_after = engine.search(&query, &candidates, 10).unwrap();
        let first_after = results_after[0].memory.incident_id.clone();

        // The feedback should have boosted inc-2
        assert_eq!(first_after, "inc-2");
    }

    #[test]
    fn hybrid_search_without_qdrant_falls_back() {
        let engine = RetrievalEngine::new();
        let query = make_query("Test", vec!["T1059"], vec!["test"]);
        let candidates = vec![make_memory(
            "inc-1",
            "Test summary",
            vec!["T1059"],
            vec!["test"],
        )];

        let rt = tokio::runtime::Runtime::new().unwrap();
        let results = rt
            .block_on(engine.search_hybrid(&query, &candidates, "test query", 10))
            .unwrap();
        assert_eq!(results.len(), 1);
    }
}
