use crate::context::{ContextEngine, ContextWeight};
use crate::reasoning::{ReasoningEngine, ReasoningResult};
use odin_kernel::{Confidence, Evidence, IntelligenceObject, KernelError};

pub enum PipelineStage {
    Ingestion,
    Validation,
    Reasoning,
    Contextualization,
    Completion,
}

pub struct IntelligencePipeline {
    reasoning: ReasoningEngine,
    context: ContextEngine,
    stage: PipelineStage,
}

impl IntelligencePipeline {
    pub fn new() -> Self {
        Self {
            reasoning: ReasoningEngine::new(),
            context: ContextEngine::new(),
            stage: PipelineStage::Ingestion,
        }
    }

    pub fn process_evidence(
        &mut self,
        evidence: &[Evidence],
    ) -> Result<PipelineResult, KernelError> {
        self.stage = PipelineStage::Validation;
        for e in evidence {
            e.validate()?;
        }
        self.stage = PipelineStage::Reasoning;
        let trust_scores: Vec<(String, f64)> = evidence
            .iter()
            .map(|e| (e.id.clone(), e.trust_score))
            .collect();
        let results = self.reasoning.evaluate(&trust_scores);
        self.stage = PipelineStage::Contextualization;
        let context_factors: Vec<ContextWeight> = vec![
            ContextWeight {
                factor: "severity".into(),
                weight: 0.8,
            },
            ContextWeight {
                factor: "recency".into(),
                weight: 0.9,
            },
        ];
        let contextualized: Vec<ReasoningResult> = results
            .into_iter()
            .map(|r| ReasoningResult {
                confidence: self.context.apply_context(&r.confidence, &context_factors),
                ..r
            })
            .collect();
        self.stage = PipelineStage::Completion;
        Ok(PipelineResult {
            overall_confidence: self.compute_overall_confidence(&contextualized),
            results: contextualized,
        })
    }

    fn compute_overall_confidence(&self, results: &[ReasoningResult]) -> Confidence {
        let sources: Vec<odin_kernel::ConfidenceSource> = results
            .iter()
            .flat_map(|r| r.confidence.sources.clone())
            .collect();
        Confidence::new(sources)
    }

    pub fn stage(&self) -> &PipelineStage {
        &self.stage
    }
}

impl Default for IntelligencePipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct PipelineResult {
    pub results: Vec<ReasoningResult>,
    pub overall_confidence: Confidence,
}

#[cfg(test)]
mod tests {
    use super::*;
    use odin_kernel::{Evidence, EvidenceType};

    fn make_ev(source: &str, trust: f64) -> Evidence {
        Evidence::new(
            "inc-1".into(),
            source.into(),
            "content".into(),
            EvidenceType::Log,
            trust,
        )
    }

    #[test]
    fn process_empty_evidence() {
        let mut pipeline = IntelligencePipeline::new();
        let result = pipeline.process_evidence(&[]).unwrap();
        assert!(result.results.is_empty());
        assert!(matches!(pipeline.stage(), PipelineStage::Completion));
    }

    #[test]
    fn process_single_evidence_returns_results() {
        let mut pipeline = IntelligencePipeline::new();
        let ev = make_ev("firewall.log", 0.9);
        let result = pipeline.process_evidence(&[ev]).unwrap();
        assert_eq!(result.results.len(), 4);
        assert!(result.overall_confidence.score > 0.0);
    }

    #[test]
    fn process_multiple_evidence() {
        let mut pipeline = IntelligencePipeline::new();
        let ev1 = make_ev("log1", 0.9);
        let ev2 = make_ev("log2", 0.7);
        let result = pipeline.process_evidence(&[ev1, ev2]).unwrap();
        assert!(!result.results.is_empty());
    }

    #[test]
    fn process_invalid_evidence_fails() {
        let mut pipeline = IntelligencePipeline::new();
        let ev = Evidence::new("i".into(), "".into(), "c".into(), EvidenceType::Log, 0.5);
        assert!(pipeline.process_evidence(&[ev]).is_err());
    }

    #[test]
    fn stage_transitions_through_pipeline() {
        let mut pipeline = IntelligencePipeline::new();
        assert!(matches!(pipeline.stage(), PipelineStage::Ingestion));
        let ev = make_ev("s", 0.8);
        pipeline.process_evidence(&[ev]).unwrap();
        assert!(matches!(pipeline.stage(), PipelineStage::Completion));
    }

    #[test]
    fn results_have_matched_rules() {
        let mut pipeline = IntelligencePipeline::new();
        let ev = make_ev("s", 0.9);
        let result = pipeline.process_evidence(&[ev]).unwrap();
        for r in &result.results {
            assert!(!r.matched_rules.is_empty());
        }
    }
}
