use crate::similarity::HybridScore;
use odin_kernel::MemoryObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedResult {
    pub memory: MemoryObject,
    pub score: HybridScore,
    pub reasons: Vec<String>,
}

pub struct Ranker;

impl Ranker {
    pub fn new() -> Self {
        Self
    }

    pub fn rank(
        results: Vec<(MemoryObject, HybridScore)>,
    ) -> Vec<RankedResult> {
        let mut ranked: Vec<RankedResult> = results
            .into_iter()
            .map(|(memory, score)| {
                let reasons = Self::generate_reasons(&score);
                RankedResult {
                    memory,
                    score,
                    reasons,
                }
            })
            .collect();
        ranked.sort_by(|a, b| b.score.overall.partial_cmp(&a.score.overall).unwrap_or(std::cmp::Ordering::Equal));
        ranked
    }

    fn generate_reasons(score: &HybridScore) -> Vec<String> {
        let mut reasons = Vec::new();
        if score.structural > 0.5 {
            reasons.push(format!("Strong structural match ({:.0}%)", score.structural * 100.0));
        }
        if score.semantic > 0.5 {
            reasons.push(format!("Strong semantic match ({:.0}%)", score.semantic * 100.0));
        }
        if score.context > 0.5 {
            reasons.push(format!("Relevant context match ({:.0}%)", score.context * 100.0));
        }
        if reasons.is_empty() {
            reasons.push("No strong match factors found".into());
        }
        reasons
    }
}

impl Default for Ranker {
    fn default() -> Self {
        Self::new()
    }
}
