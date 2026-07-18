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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contain_high_confidence_priority_is_1() {
        let sources = vec![ConfidenceSource {
            label: "a".into(),
            trust: 0.95,
        }];
        let rec = Recommendation::new(
            RecommendationKind::Contain,
            "T".into(),
            "D".into(),
            vec![],
            sources,
        );
        assert_eq!(rec.priority, 1);
    }

    #[test]
    fn contain_low_confidence_priority_is_3() {
        let sources = vec![ConfidenceSource {
            label: "a".into(),
            trust: 0.4,
        }];
        let rec = Recommendation::new(
            RecommendationKind::Contain,
            "T".into(),
            "D".into(),
            vec![],
            sources,
        );
        assert_eq!(rec.priority, 3);
    }

    #[test]
    fn investigate_medium_confidence_priority_is_4() {
        let sources = vec![ConfidenceSource {
            label: "a".into(),
            trust: 0.6,
        }];
        let rec = Recommendation::new(
            RecommendationKind::Investigate,
            "T".into(),
            "D".into(),
            vec![],
            sources,
        );
        assert_eq!(rec.priority, 4);
    }

    #[test]
    fn decision_serialization_roundtrip() {
        let decision = Decision {
            recommendations: vec![],
            overall_confidence: Confidence::default(),
            supporting_evidence: vec!["ev-1".into()],
        };
        let json = serde_json::to_string(&decision).unwrap();
        let deserialized: Decision = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.supporting_evidence[0], "ev-1");
    }
}
