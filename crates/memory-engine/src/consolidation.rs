use odin_kernel::{KernelError, MemoryObject};
use std::sync::Arc;

pub type Summarizer = Arc<dyn Fn(&str) -> Result<String, KernelError> + Send + Sync>;

pub struct ConsolidationConfig {
    pub default_ttl_days: u64,
    pub max_versions_per_memory: usize,
    pub consolidation_interval_minutes: u64,
    pub summarizer: Option<Summarizer>,
}

impl ConsolidationConfig {
    pub fn new() -> Self {
        Self {
            default_ttl_days: 90,
            max_versions_per_memory: 10,
            consolidation_interval_minutes: 60,
            summarizer: None,
        }
    }

    pub fn with_summarizer<F>(summarizer: F) -> Self
    where
        F: Fn(&str) -> Result<String, KernelError> + Send + Sync + 'static,
    {
        Self {
            default_ttl_days: 90,
            max_versions_per_memory: 10,
            consolidation_interval_minutes: 60,
            summarizer: Some(Arc::new(summarizer)),
        }
    }

    pub fn ttl_for_severity(severity: &str) -> chrono::TimeDelta {
        match severity.to_lowercase().as_str() {
            "critical" => chrono::TimeDelta::days(365),
            "high" => chrono::TimeDelta::days(180),
            "medium" => chrono::TimeDelta::days(90),
            "low" => chrono::TimeDelta::days(30),
            _ => chrono::TimeDelta::days(90),
        }
    }

    pub fn set_memory_ttl(memory: &mut MemoryObject, severity: &str) {
        let ttl = Self::ttl_for_severity(severity);
        memory.expires_at = Some(chrono::Utc::now() + ttl);
    }
}

impl Default for ConsolidationConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConsolidationReport {
    pub expired_count: usize,
    pub pruned_version_count: usize,
    pub consolidated_memories: Vec<String>,
}

impl ConsolidationReport {
    pub fn is_empty(&self) -> bool {
        self.expired_count == 0 && self.pruned_version_count == 0 && self.consolidated_memories.is_empty()
    }
}
