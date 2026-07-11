use odin_core::odin_kernel::{CanonicalIncident, Evidence, Entity, KernelError};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use odin_core::odin_memory_engine::{ConsolidationConfig, MemoryEngine, PgStore};
use odin_core::odin_intelligence_engine::{IntelligenceEngine, OllamaPipeline};
use odin_core::odin_retrieval_engine::{RetrievalEngine, QdrantClient};
use odin_core::odin_decision_engine::DecisionEngine;
use odin_core::odin_policy_gate::PolicyGate;
use odin_core::odin_infrastructure::{InfrastructureConfig, OllamaClient, RedisClient, Neo4jClient, JwtService, AuditLogger};

pub type IncidentMap = Arc<RwLock<HashMap<String, CanonicalIncident>>>;
pub type EvidenceMap = Arc<RwLock<HashMap<String, Vec<Evidence>>>>;
pub type EntityMap = Arc<RwLock<HashMap<String, Vec<Entity>>>>;
pub type FeedbackMap = Arc<RwLock<HashMap<String, Vec<FeedbackEntry>>>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FeedbackEntry {
    pub feedback: String,
    pub rating: u8,
    pub created_at: String,
}

pub struct AppState {
    pub incidents: IncidentMap,
    pub evidence: EvidenceMap,
    pub entities: EntityMap,
    pub feedback: FeedbackMap,
    pub memory: MemoryEngine,
    pub intelligence: Arc<RwLock<IntelligenceEngine>>,
    pub retrieval: RetrievalEngine,
    pub decision: DecisionEngine,
    pub policy: PolicyGate,
    pub pg_store: Option<PgStore>,
    pub qdrant: Option<QdrantClient>,
    pub ollama_client: Option<OllamaClient>,
    pub redis: Option<RedisClient>,
    pub neo4j: Option<Neo4jClient>,
    pub jwt_service: JwtService,
    pub audit_logger: AuditLogger,
    #[allow(dead_code)]
    pub infra_config: InfrastructureConfig,
}

impl AppState {
    pub fn new() -> Self {
        let config = InfrastructureConfig::from_env();
        let ollama_client = Some(OllamaClient::new(
            &config.ollama_url,
            &config.ollama_embed_model,
            &config.ollama_reason_model,
        ));

        let pg_store = None;
        let qdrant = None;

        let ollama_pipeline = ollama_client.as_ref().map(|oc| {
            OllamaPipeline::new(oc.clone())
        });

        Self {
            incidents: Arc::new(RwLock::new(HashMap::new())),
            evidence: Arc::new(RwLock::new(HashMap::new())),
            entities: Arc::new(RwLock::new(HashMap::new())),
            feedback: Arc::new(RwLock::new(HashMap::new())),
            memory: MemoryEngine::new(),
            intelligence: Arc::new(RwLock::new(
                match ollama_pipeline {
                    Some(ollama) => IntelligenceEngine::with_ollama(ollama),
                    None => IntelligenceEngine::new(),
                }
            )),
            retrieval: RetrievalEngine::new(),
            decision: DecisionEngine::new(),
            policy: PolicyGate::new(),
            pg_store,
            qdrant,
            ollama_client,
            redis: None,
            neo4j: None,
            jwt_service: JwtService::new("odin-secret-key-change-in-production"),
            audit_logger: AuditLogger::new(10000),
            infra_config: config,
        }
    }

    pub async fn connect_database(&mut self) {
        let config = InfrastructureConfig::from_env();
        let ollama_client = OllamaClient::new(
            &config.ollama_url,
            &config.ollama_embed_model,
            &config.ollama_reason_model,
        );

        match PgStore::connect(&config.database_url).await {
            Ok(store) => {
                if let Err(e) = store.run_migrations().await {
                    tracing::error!("Migration failed: {}", e);
                }
                self.pg_store = Some(store.clone());

                let ollama_for_summary = ollama_client.clone();
                let config = ConsolidationConfig::with_summarizer(move |prompt: &str| {
                    let client = ollama_for_summary.clone();
                    let p = prompt.to_string();
                    tokio::runtime::Handle::try_current()
                        .map_err(|_| KernelError::Internal("No tokio runtime".into()))?
                        .block_on(async move { client.generate(&p, 0.3).await })
                });
                let engine = MemoryEngine::with_store(Box::new(store)).with_consolidation(config);

                self.memory = engine;
                tracing::info!("PostgreSQL connected and migrated");
            }
            Err(e) => {
                tracing::warn!("PostgreSQL not available, using in-memory store: {}", e);
            }
        }

        let qdrant = QdrantClient::new(&config.qdrant_url, "incidents");
        match qdrant.ensure_collection(768).await {
            Ok(()) => {
                let qdrant_for_retrieval = qdrant.clone();
                let ollama = ollama_client.clone();
                let embed_ollama = ollama.clone();
                self.retrieval = RetrievalEngine::with_qdrant(
                    qdrant_for_retrieval,
                    Box::new(move |text: &str| {
                        let client = embed_ollama.clone();
                        let t = text.to_string();
                        tokio::runtime::Handle::try_current()
                            .unwrap()
                            .block_on(async move {
                                client.generate_embedding(&t).await
                            })
                    }),
                );
                self.qdrant = Some(qdrant);
                self.ollama_client = Some(ollama_client.clone());
                tracing::info!("Qdrant connected");
            }
            Err(e) => {
                tracing::warn!("Qdrant not available, using local scoring only: {}", e);
            }
        }

        let mut redis = RedisClient::new(&config.redis_url);
        match redis.connect().await {
            Ok(()) => {
                self.redis = Some(redis);
                tracing::info!("Redis connected");
            }
            Err(e) => {
                tracing::warn!("Redis not available: {}", e);
            }
        }

        match Neo4jClient::new(&config.neo4j_url, &config.neo4j_user, &config.neo4j_password).await {
            Ok(client) => {
                self.neo4j = Some(client);
                tracing::info!("Neo4j connected");
            }
            Err(e) => {
                tracing::warn!("Neo4j not available, using in-memory graph: {}", e);
            }
        }
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}
