use odin_infrastructure::OllamaClient;
use odin_kernel::{Evidence, KernelError};

pub struct OllamaPipeline {
    client: OllamaClient,
}

impl OllamaPipeline {
    pub fn new(client: OllamaClient) -> Self {
        Self { client }
    }

    pub async fn analyze_evidence(&self, evidence: &[Evidence]) -> Result<OllamaAnalysis, KernelError> {
        let evidence_text: String = evidence
            .iter()
            .map(|e| format!("[Source: {}]\n{}", e.source, e.content))
            .collect::<Vec<_>>()
            .join("\n---\n");

        let analysis = self.client.analyze_evidence(&evidence_text).await?;
        Ok(OllamaAnalysis {
            raw_analysis: analysis,
        })
    }

    pub async fn generate_narrative(
        &self,
        summary: &str,
        techniques: &[String],
    ) -> Result<String, KernelError> {
        self.client.generate_narrative(summary, techniques).await
    }

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f64>, KernelError> {
        self.client.generate_embedding(text).await
    }
}

#[derive(Debug, Clone)]
pub struct OllamaAnalysis {
    pub raw_analysis: String,
}
