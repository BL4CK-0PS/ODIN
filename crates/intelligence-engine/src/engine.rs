use crate::ollama_pipeline::OllamaPipeline;
use crate::pipeline::{IntelligencePipeline, PipelineResult};
use odin_kernel::{Evidence, KernelError};

pub struct IntelligenceEngine {
    pipeline: IntelligencePipeline,
    ollama: Option<OllamaPipeline>,
}

impl IntelligenceEngine {
    pub fn new() -> Self {
        Self {
            pipeline: IntelligencePipeline::new(),
            ollama: None,
        }
    }

    pub fn with_ollama(ollama: OllamaPipeline) -> Self {
        Self {
            pipeline: IntelligencePipeline::new(),
            ollama: Some(ollama),
        }
    }

    pub fn analyze(&mut self, evidence: &[Evidence]) -> Result<PipelineResult, KernelError> {
        self.pipeline.process_evidence(evidence)
    }

    pub async fn analyze_with_ollama(
        &mut self,
        evidence: &[Evidence],
    ) -> Result<PipelineResult, KernelError> {
        let mut result = self.pipeline.process_evidence(evidence)?;
        if let Some(ref ollama) = self.ollama {
            let analysis = ollama.analyze_evidence(evidence).await?;
            for r in &mut result.results {
                r.reasoning_steps
                    .push(format!("Ollama analysis: {}", analysis.raw_analysis));
            }
        }
        Ok(result)
    }

    pub async fn generate_narrative(
        &self,
        summary: &str,
        techniques: &[String],
    ) -> Result<String, KernelError> {
        match self.ollama {
            Some(ref ollama) => ollama.generate_narrative(summary, techniques).await,
            None => Ok("Ollama pipeline not configured".to_string()),
        }
    }

    pub fn has_ollama(&self) -> bool {
        self.ollama.is_some()
    }
}

impl Default for IntelligenceEngine {
    fn default() -> Self {
        Self::new()
    }
}
