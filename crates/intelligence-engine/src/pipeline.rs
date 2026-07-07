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

    pub fn process_evidence(&mut self, evidence: &[Evidence]) -> Result<PipelineResult, KernelError> {
        self.stage = PipelineStage::Validation;
        for e in evidence {
            e.validate()?;
        }
        self.stage = PipelineStage::Reasoning;
        let trust_scores: Vec<(String, f64)> =
            evidence.iter().map(|e| (e.id.clone(), e.trust_score)).collect();
        let results = self.reasoning.evaluate(&trust_scores);
        self.stage = PipelineStage::Contextualization;
        let context_factors: Vec<ContextWeight> = vec![
            ContextWeight { factor: "severity".into(), weight: 0.8 },
            ContextWeight { factor: "recency".into(), weight: 0.9 },
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
