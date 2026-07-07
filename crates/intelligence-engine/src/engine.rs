use crate::pipeline::{IntelligencePipeline, PipelineResult};
use odin_kernel::{Evidence, KernelError};

pub struct IntelligenceEngine {
    pipeline: IntelligencePipeline,
}

impl IntelligenceEngine {
    pub fn new() -> Self {
        Self {
            pipeline: IntelligencePipeline::new(),
        }
    }

    pub fn analyze(&mut self, evidence: &[Evidence]) -> Result<PipelineResult, KernelError> {
        self.pipeline.process_evidence(evidence)
    }
}

impl Default for IntelligenceEngine {
    fn default() -> Self {
        Self::new()
    }
}
