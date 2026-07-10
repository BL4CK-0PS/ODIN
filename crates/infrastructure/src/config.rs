use std::env;

#[derive(Debug, Clone)]
pub struct InfrastructureConfig {
    pub database_url: String,
    pub qdrant_url: String,
    pub ollama_url: String,
    pub ollama_embed_model: String,
    pub ollama_reason_model: String,
}

impl InfrastructureConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_|
                "postgresql://odin:odin@localhost:5432/odin".to_string()
            ),
            qdrant_url: env::var("QDRANT_URL").unwrap_or_else(|_|
                "http://localhost:6334".to_string()
            ),
            ollama_url: env::var("OLLAMA_URL").unwrap_or_else(|_|
                "http://localhost:11434".to_string()
            ),
            ollama_embed_model: env::var("OLLAMA_EMBED_MODEL").unwrap_or_else(|_|
                "nomic-embed-text".to_string()
            ),
            ollama_reason_model: env::var("OLLAMA_REASON_MODEL").unwrap_or_else(|_|
                "qwen3:8b".to_string()
            ),
        }
    }
}
