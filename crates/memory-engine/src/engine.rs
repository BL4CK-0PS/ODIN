use crate::builder::MemoryBuilder;
use crate::consolidation::{ConsolidationConfig, ConsolidationReport};
use crate::store::{InMemoryStore, MemoryStore};
use crate::version::MemoryVersion;
use odin_kernel::{CanonicalIncident, KernelError, MemoryObject};

pub struct MemoryEngine {
    builder: MemoryBuilder,
    store: Box<dyn MemoryStore>,
    consolidation_config: Option<ConsolidationConfig>,
}

impl MemoryEngine {
    pub fn new() -> Self {
        Self {
            builder: MemoryBuilder::new(),
            store: Box::new(InMemoryStore::new()),
            consolidation_config: None,
        }
    }

    pub fn with_store(store: Box<dyn MemoryStore>) -> Self {
        Self {
            builder: MemoryBuilder::new(),
            store,
            consolidation_config: None,
        }
    }

    pub fn with_consolidation(mut self, config: ConsolidationConfig) -> Self {
        self.consolidation_config = Some(config);
        self
    }

    pub fn store_incident(&self, incident: &CanonicalIncident) -> Result<MemoryObject, KernelError> {
        let mut memory = self.builder.build(incident)?;

        if self.consolidation_config.is_some() {
            let sev = format!("{:?}", incident.severity);
            ConsolidationConfig::set_memory_ttl(&mut memory, &sev);
        }

        self.store.save(memory.clone())?;
        let version = MemoryVersion::new(memory.clone(), "initial".into());
        self.store.save_version(version)?;
        Ok(memory)
    }

    pub fn get_memory(&self, id: &str) -> Result<Option<MemoryObject>, KernelError> {
        self.store.find_by_id(id)
    }

    pub fn get_memory_by_incident(&self, incident_id: &str) -> Result<Option<MemoryObject>, KernelError> {
        self.store.find_by_incident_id(incident_id)
    }

    pub fn list_all(&self) -> Result<Vec<MemoryObject>, KernelError> {
        self.store.list_all()
    }

    pub fn get_versions(&self, memory_id: &str) -> Result<Vec<MemoryVersion>, KernelError> {
        self.store.get_versions(memory_id)
    }

    pub fn purge_expired(&self) -> Result<Vec<String>, KernelError> {
        self.store.purge_expired()
    }

    pub fn prune_versions(&self, memory_id: &str, max_versions: usize) -> Result<usize, KernelError> {
        self.store.prune_versions(memory_id, max_versions)
    }

    pub fn run_consolidation(&self) -> Result<ConsolidationReport, KernelError> {
        let expired = self.store.purge_expired()?;
        let memories = self.store.list_all()?;
        let mut total_pruned = 0;
        for memory in &memories {
            total_pruned += self.store.prune_versions(&memory.id, 10)?;
        }

        let mut consolidated = Vec::new();
        if let Some(ref config) = self.consolidation_config {
            let cutoff = chrono::Utc::now() - chrono::TimeDelta::days(7);
            let old: Vec<&MemoryObject> = memories.iter().filter(|m| m.created_at < cutoff).collect();
            if let Some(ref summarizer) = config.summarizer {
                for memory in old {
                    let prompt = format!(
                        "Condense the following incident summary into a concise 1-2 sentence overview:\n\n{}",
                        memory.summary
                    );
                    match summarizer(&prompt) {
                        Ok(condensed) => {
                            let mut consolidated_memory = MemoryObject::new(
                                memory.incident_id.clone(),
                                condensed,
                                memory.context.clone(),
                                memory.confidence,
                            );
                            consolidated_memory.expires_at = memory.expires_at;
                            if self.store.save(consolidated_memory).is_ok() {
                                consolidated.push(memory.id.clone());
                            }
                        }
                        Err(e) => tracing::warn!("Summarization failed: {}", e),
                    }
                }
            }
        }

        Ok(ConsolidationReport {
            expired_count: expired.len(),
            pruned_version_count: total_pruned,
            consolidated_memories: consolidated,
        })
    }
}

impl Default for MemoryEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for MemoryEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryEngine").finish()
    }
}
