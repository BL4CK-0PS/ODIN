use odin_kernel::KernelError;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct OllamaClient {
    base_url: String,
    embed_model: String,
    reason_model: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct EmbedRequest {
    model: String,
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct EmbedResponse {
    embedding: Vec<f64>,
}

#[derive(Debug, Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: Option<GenerateOptions>,
}

#[derive(Debug, Serialize)]
struct GenerateOptions {
    temperature: f64,
    top_p: f64,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
    #[allow(dead_code)]
    done: bool,
}

impl OllamaClient {
    pub fn new(base_url: &str, embed_model: &str, reason_model: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            embed_model: embed_model.to_string(),
            reason_model: reason_model.to_string(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f64>, KernelError> {
        let url = format!("{}/api/embeddings", self.base_url);
        let body = EmbedRequest {
            model: self.embed_model.clone(),
            prompt: text.to_string(),
        };
        let resp = self.client.post(&url).json(&body).send().await.map_err(|e| {
            KernelError::Internal(format!("Ollama embed request failed: {}", e))
        })?;
        let data: EmbedResponse = resp.json().await.map_err(|e| {
            KernelError::Internal(format!("Ollama embed parse failed: {}", e))
        })?;
        Ok(data.embedding)
    }

    pub async fn generate(&self, prompt: &str, temperature: f64) -> Result<String, KernelError> {
        let url = format!("{}/api/generate", self.base_url);
        let body = GenerateRequest {
            model: self.reason_model.clone(),
            prompt: prompt.to_string(),
            stream: false,
            options: Some(GenerateOptions {
                temperature,
                top_p: 0.9,
            }),
        };
        let resp = self.client.post(&url).json(&body).send().await.map_err(|e| {
            KernelError::Internal(format!("Ollama generate request failed: {}", e))
        })?;
        let data: GenerateResponse = resp.json().await.map_err(|e| {
            KernelError::Internal(format!("Ollama generate parse failed: {}", e))
        })?;
        Ok(data.response)
    }

    pub async fn analyze_evidence(&self, evidence_text: &str) -> Result<String, KernelError> {
        let prompt = format!(
            "You are a cybersecurity analyst. Analyze the following evidence and identify:\n\
            1. Key indicators of compromise (IPs, domains, hashes, processes)\n\
            2. MITRE ATT&CK techniques likely involved\n\
            3. Severity assessment (Critical/High/Medium/Low/Informational)\n\
            4. Brief description of the attack pattern\n\n\
            Evidence:\n{}\n\n\
            Provide a concise analysis.",
            evidence_text
        );
        self.generate(&prompt, 0.3).await
    }

    pub async fn generate_narrative(&self, incident_summary: &str, techniques: &[String]) -> Result<String, KernelError> {
        let techs = techniques.join(", ");
        let prompt = format!(
            "Generate a concise attack narrative for a cybersecurity incident with the following details:\n\
            Summary: {}\n\
            MITRE Techniques: {}\n\n\
            Describe the likely attack chain, initial access method, persistence mechanism, \
            lateral movement technique, and impact. Be specific and technical.",
            incident_summary, techs
        );
        self.generate(&prompt, 0.4).await
    }

    pub async fn extract_entities_with_ai(
        &self,
        evidence_text: &str,
        evidence_type: &str,
    ) -> Result<String, KernelError> {
        let prompt = format!(
            "Extract structured entities from this cybersecurity evidence.\n\
            Evidence type: {}\n\n\
            Evidence content:\n{}\n\n\
            Return a JSON object with these arrays:\n\
            - \"ip_addresses\": list of IP addresses found\n\
            - \"domains\": list of domains/hostnames found\n\
            - \"file_hashes\": list of file hashes (MD5/SHA1/SHA256)\n\
            - \"urls\": list of URLs found\n\
            - \"mitre_techniques\": list of MITRE ATT&CK technique IDs (T1xxx format)\n\
            - \"processes\": list of process names or paths\n\
            - \"malware_families\": list of malware family names if identifiable\n\n\
            Return ONLY the JSON object, no other text. Use empty arrays for categories with no findings.",
            evidence_type, evidence_text
        );
        self.generate(&prompt, 0.1).await
    }
}
