use odin_kernel::KernelError;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct QdrantClient {
    base_url: String,
    client: reqwest::Client,
    collection: String,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    result: Vec<ScoredPoint>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScoredPoint {
    pub id: String,
    pub score: f64,
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct SearchRequest {
    vector: Vec<f64>,
    limit: usize,
    with_payload: bool,
}

impl QdrantClient {
    pub fn new(base_url: &str, collection: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            collection: collection.to_string(),
        }
    }

    pub async fn ensure_collection(&self, vector_size: u64) -> Result<(), KernelError> {
        let url = format!("{}/collections/{}", self.base_url, self.collection);
        let exists =
            self.client.get(&url).send().await.map_err(|e| {
                KernelError::Internal(format!("Qdrant check collection failed: {}", e))
            })?;
        if exists.status().is_success() {
            return Ok(());
        }
        let create_url = format!("{}/collections", self.base_url);
        let body = json!({
            "name": self.collection,
            "vectors": {
                "size": vector_size,
                "distance": "Cosine"
            }
        });
        self.client
            .put(&create_url)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                KernelError::Internal(format!("Qdrant create collection failed: {}", e))
            })?;
        tracing::info!("Created Qdrant collection: {}", self.collection);
        Ok(())
    }

    pub async fn upsert_vector(
        &self,
        id: &str,
        vector: Vec<f64>,
        payload: serde_json::Value,
    ) -> Result<(), KernelError> {
        let url = format!("{}/collections/{}/points", self.base_url, self.collection);
        let body = json!({
            "points": [{
                "id": id,
                "vector": vector,
                "payload": payload
            }]
        });
        self.client
            .put(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| KernelError::Internal(format!("Qdrant upsert failed: {}", e)))?;
        Ok(())
    }

    pub async fn search(
        &self,
        vector: Vec<f64>,
        limit: usize,
    ) -> Result<Vec<ScoredPoint>, KernelError> {
        let url = format!(
            "{}/collections/{}/points/search",
            self.base_url, self.collection
        );
        let body = SearchRequest {
            vector,
            limit,
            with_payload: true,
        };
        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| KernelError::Internal(format!("Qdrant search failed: {}", e)))?;
        let data: SearchResponse = resp
            .json()
            .await
            .map_err(|e| KernelError::Internal(format!("Qdrant search parse failed: {}", e)))?;
        Ok(data.result)
    }

    pub async fn delete_by_id(&self, id: &str) -> Result<(), KernelError> {
        let url = format!(
            "{}/collections/{}/points/delete",
            self.base_url, self.collection
        );
        let body = json!({
            "points": [id]
        });
        self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| KernelError::Internal(format!("Qdrant delete failed: {}", e)))?;
        Ok(())
    }
}
