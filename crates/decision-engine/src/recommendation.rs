use odin_kernel::{Confidence, ConfidenceSource};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationKind {
    Contain,
    Investigate,
    Eradicate,
    Recover,
    Escalate,
    Monitor,
    Inform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub kind: RecommendationKind,
    pub title: String,
    pub description: String,
    pub confidence: Confidence,
    pub evidence_ids: Vec<String>,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub recommendations: Vec<Recommendation>,
    pub overall_confidence: Confidence,
    pub supporting_evidence: Vec<String>,
}

impl Recommendation {
    pub fn new(
        kind: RecommendationKind,
        title: String,
        description: String,
        evidence_ids: Vec<String>,
        confidence_sources: Vec<ConfidenceSource>,
    ) -> Self {
        let confidence = Confidence::new(confidence_sources);
        let priority = Self::compute_priority(&confidence, &kind);
        Self {
            kind,
            title,
            description,
            confidence,
            evidence_ids,
            priority,
        }
    }

    fn compute_priority(confidence: &Confidence, kind: &RecommendationKind) -> u8 {
        let base = match kind {
            RecommendationKind::Contain => 1,
            RecommendationKind::Eradicate => 2,
            RecommendationKind::Escalate => 2,
            RecommendationKind::Investigate => 3,
            RecommendationKind::Recover => 3,
            RecommendationKind::Monitor => 4,
            RecommendationKind::Inform => 5,
        };
        if confidence.score > 0.8 {
            base
        } else if confidence.score > 0.5 {
            base + 1
        } else {
            base + 2
        }
    }
}
