use crate::version::MemoryVersion;
use odin_kernel::{KernelError, MemoryObject};
use std::collections::HashMap;
use std::sync::RwLock;

pub trait MemoryStore: Send + Sync {
    fn save(&self, memory: MemoryObject) -> Result<(), KernelError>;
    fn find_by_id(&self, id: &str) -> Result<Option<MemoryObject>, KernelError>;
    fn find_by_incident_id(&self, incident_id: &str) -> Result<Option<MemoryObject>, KernelError>;
    fn list_all(&self) -> Result<Vec<MemoryObject>, KernelError>;
    fn save_version(&self, version: MemoryVersion) -> Result<(), KernelError>;
    fn get_versions(&self, memory_id: &str) -> Result<Vec<MemoryVersion>, KernelError>;
}

pub struct InMemoryStore {
    objects: RwLock<HashMap<String, MemoryObject>>,
    by_incident: RwLock<HashMap<String, String>>,
    versions: RwLock<HashMap<String, Vec<MemoryVersion>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            objects: RwLock::new(HashMap::new()),
            by_incident: RwLock::new(HashMap::new()),
            versions: RwLock::new(HashMap::new()),
        }
    }
}

impl MemoryStore for InMemoryStore {
    fn save(&self, memory: MemoryObject) -> Result<(), KernelError> {
        let incident_id = memory.incident_id.clone();
        let memory_id = memory.id.clone();
        self.objects.write().map_err(|e| KernelError::Internal(e.to_string()))?.insert(memory_id.clone(), memory);
        self.by_incident.write().map_err(|e| KernelError::Internal(e.to_string()))?.insert(incident_id, memory_id);
        Ok(())
    }

    fn find_by_id(&self, id: &str) -> Result<Option<MemoryObject>, KernelError> {
        Ok(self.objects.read().map_err(|e| KernelError::Internal(e.to_string()))?.get(id).cloned())
    }

    fn find_by_incident_id(&self, incident_id: &str) -> Result<Option<MemoryObject>, KernelError> {
        let by_incident = self.by_incident.read().map_err(|e| KernelError::Internal(e.to_string()))?;
        if let Some(memory_id) = by_incident.get(incident_id) {
            self.find_by_id(memory_id)
        } else {
            Ok(None)
        }
    }

    fn list_all(&self) -> Result<Vec<MemoryObject>, KernelError> {
        Ok(self.objects.read().map_err(|e| KernelError::Internal(e.to_string()))?.values().cloned().collect())
    }

    fn save_version(&self, version: MemoryVersion) -> Result<(), KernelError> {
        let memory_id = version.memory.id.clone();
        let mut versions = self.versions.write().map_err(|e| KernelError::Internal(e.to_string()))?;
        versions.entry(memory_id).or_default().push(version);
        Ok(())
    }

    fn get_versions(&self, memory_id: &str) -> Result<Vec<MemoryVersion>, KernelError> {
        Ok(self.versions.read().map_err(|e| KernelError::Internal(e.to_string()))?.get(memory_id).cloned().unwrap_or_default())
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for InMemoryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InMemoryStore").finish()
    }
}
