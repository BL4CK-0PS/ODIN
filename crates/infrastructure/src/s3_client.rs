use odin_kernel::KernelError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct S3Client {
    endpoint: String,
    bucket: String,
    access_key: String,
    secret_key: String,
    client: reqwest::Client,
}

impl S3Client {
    pub fn new(endpoint: &str, bucket: &str, access_key: &str, secret_key: &str) -> Self {
        Self {
            endpoint: endpoint.trim_end_matches('/').to_string(),
            bucket: bucket.to_string(),
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn health_check(&self) -> bool {
        let url = format!("{}/{}", self.endpoint, self.bucket);
        self.client
            .head(&url)
            .basic_auth(&self.access_key, Some(&self.secret_key))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    pub async fn put_object(&self, key: &str, data: &[u8]) -> Result<String, KernelError> {
        let url = format!("{}/{}/{}", self.endpoint, self.bucket, key);
        let response = self
            .client
            .put(&url)
            .basic_auth(&self.access_key, Some(&self.secret_key))
            .header("Content-Type", "application/octet-stream")
            .body(data.to_vec())
            .send()
            .await
            .map_err(|e| KernelError::Internal(format!("S3 put failed: {}", e)))?;

        let etag = response
            .headers()
            .get("etag")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        Ok(etag)
    }

    pub async fn get_object(&self, key: &str) -> Result<Vec<u8>, KernelError> {
        let url = format!("{}/{}/{}", self.endpoint, self.bucket, key);
        let response = self
            .client
            .get(&url)
            .basic_auth(&self.access_key, Some(&self.secret_key))
            .send()
            .await
            .map_err(|e| KernelError::Internal(format!("S3 get failed: {}", e)))?;

        let bytes = response
            .bytes()
            .await
            .map_err(|e| KernelError::Internal(format!("S3 read body failed: {}", e)))?;

        Ok(bytes.to_vec())
    }

    pub async fn delete_object(&self, key: &str) -> Result<(), KernelError> {
        let url = format!("{}/{}/{}", self.endpoint, self.bucket, key);
        self.client
            .delete(&url)
            .basic_auth(&self.access_key, Some(&self.secret_key))
            .send()
            .await
            .map_err(|e| KernelError::Internal(format!("S3 delete failed: {}", e)))?;
        Ok(())
    }

    pub async fn list_objects(&self, prefix: &str) -> Result<Vec<String>, KernelError> {
        let url = format!("{}/{}?prefix={}", self.endpoint, self.bucket, prefix);
        let response = self
            .client
            .get(&url)
            .basic_auth(&self.access_key, Some(&self.secret_key))
            .send()
            .await
            .map_err(|e| KernelError::Internal(format!("S3 list failed: {}", e)))?;

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| KernelError::Internal(format!("S3 list parse failed: {}", e)))?;

        let keys = body["Contents"]
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| item["Key"].as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        Ok(keys)
    }

    pub fn evidence_key(incident_id: &str, evidence_id: &str) -> String {
        format!("evidence/{}/{}", incident_id, evidence_id)
    }
}

#[derive(Debug, Clone)]
pub struct ArtifactStore {
    s3: Option<S3Client>,
    local_cache: std::sync::Arc<tokio::sync::RwLock<HashMap<String, Vec<u8>>>>,
}

impl ArtifactStore {
    pub fn new(s3: Option<S3Client>) -> Self {
        Self {
            s3,
            local_cache: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    pub async fn store_artifact(
        &self,
        incident_id: &str,
        evidence_id: &str,
        content: &[u8],
    ) -> Result<String, KernelError> {
        let key = S3Client::evidence_key(incident_id, evidence_id);

        if let Some(ref s3) = self.s3 {
            match s3.put_object(&key, content).await {
                Ok(etag) => {
                    tracing::debug!("Stored artifact in S3: {} (etag: {})", key, etag);
                    return Ok(key);
                }
                Err(e) => {
                    tracing::warn!("S3 upload failed, falling back to local cache: {}", e);
                }
            }
        }

        let mut cache = self.local_cache.write().await;
        cache.insert(key.clone(), content.to_vec());
        Ok(key)
    }

    pub async fn get_artifact(
        &self,
        incident_id: &str,
        evidence_id: &str,
    ) -> Result<Vec<u8>, KernelError> {
        let key = S3Client::evidence_key(incident_id, evidence_id);

        if let Some(ref s3) = self.s3 {
            match s3.get_object(&key).await {
                Ok(data) => return Ok(data),
                Err(e) => {
                    tracing::warn!("S3 get failed, trying local cache: {}", e);
                }
            }
        }

        let cache = self.local_cache.read().await;
        cache
            .get(&key)
            .cloned()
            .ok_or_else(|| KernelError::Internal(format!("Artifact not found: {}", key)))
    }

    pub async fn delete_artifact(
        &self,
        incident_id: &str,
        evidence_id: &str,
    ) -> Result<(), KernelError> {
        let key = S3Client::evidence_key(incident_id, evidence_id);

        if let Some(ref s3) = self.s3 {
            let _ = s3.delete_object(&key).await;
        }

        let mut cache = self.local_cache.write().await;
        cache.remove(&key);
        Ok(())
    }

    pub fn is_s3_available(&self) -> bool {
        self.s3.is_some()
    }
}
