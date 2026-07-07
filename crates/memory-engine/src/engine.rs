use crate::builder::MemoryBuilder;
use crate::store::{InMemoryStore, MemoryStore};
use crate::version::MemoryVersion;
use odin_kernel::{CanonicalIncident, KernelError, MemoryObject};

pub struct MemoryEngine {
    builder: MemoryBuilder,
    store: Box<dyn MemoryStore>,
}

impl MemoryEngine {
    pub fn new() -> Self {
        Self {
            builder: MemoryBuilder::new(),
            store: Box::new(InMemoryStore::new()),
        }
    }

    pub fn with_store(store: Box<dyn MemoryStore>) -> Self {
        Self {
            builder: MemoryBuilder::new(),
            store,
        }
    }

    pub fn store_incident(&self, incident: &CanonicalIncident) -> Result<MemoryObject, KernelError> {
        let memory = self.builder.build(incident)?;
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
