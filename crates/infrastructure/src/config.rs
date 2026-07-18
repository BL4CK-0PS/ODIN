use std::env;

#[derive(Debug, Clone)]
pub struct InfrastructureConfig {
    pub database_url: String,
    pub redis_url: String,
    pub qdrant_url: String,
    pub ollama_url: String,
    pub ollama_embed_model: String,
    pub ollama_reason_model: String,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub neo4j_url: String,
    pub neo4j_user: String,
    pub neo4j_password: String,
}

impl InfrastructureConfig {
    pub fn from_env() -> Self {
        let s3_access_key = env::var("S3_ACCESS_KEY").unwrap_or_else(|_| {
            tracing::warn!("S3_ACCESS_KEY not set — using fallback for local development only");
            "minioadmin".to_string()
        });
        let s3_secret_key = env::var("S3_SECRET_KEY").unwrap_or_else(|_| {
            tracing::warn!("S3_SECRET_KEY not set — using fallback for local development only");
            "minioadmin".to_string()
        });
        let neo4j_password = env::var("NEO4J_PASSWORD").unwrap_or_else(|_| {
            tracing::warn!("NEO4J_PASSWORD not set — using fallback for local development only");
            "odin".to_string()
        });

        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://odin:odin@localhost:5432/odin".to_string()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            qdrant_url: env::var("QDRANT_URL")
                .unwrap_or_else(|_| "http://localhost:6334".to_string()),
            ollama_url: env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
            ollama_embed_model: env::var("OLLAMA_EMBED_MODEL")
                .unwrap_or_else(|_| "nomic-embed-text".to_string()),
            ollama_reason_model: env::var("OLLAMA_REASON_MODEL")
                .unwrap_or_else(|_| "qwen3:8b".to_string()),
            s3_endpoint: env::var("S3_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".to_string()),
            s3_bucket: env::var("S3_BUCKET").unwrap_or_else(|_| "odin-artifacts".to_string()),
            s3_access_key,
            s3_secret_key,
            neo4j_url: env::var("NEO4J_URL")
                .unwrap_or_else(|_| "neo4j://localhost:7687".to_string()),
            neo4j_user: env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string()),
            neo4j_password,
        }
    }

    /// Returns true if running with default localhost URLs (development mode).
    pub fn is_dev_defaults(&self) -> bool {
        self.database_url.contains("localhost")
            || self.s3_endpoint.contains("localhost")
            || self.neo4j_url.contains("localhost")
    }
}
