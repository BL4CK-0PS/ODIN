use odin_kernel::{CanonicalIncident, MemoryObject};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridScore {
    pub overall: f64,
    pub structural: f64,
    pub semantic: f64,
    pub context: f64,
}

pub struct StructuralScorer;

impl StructuralScorer {
    pub fn new() -> Self {
        Self
    }

    pub fn score(&self, query: &CanonicalIncident, candidate: &MemoryObject) -> f64 {
        let query_techniques: HashSet<&str> = query.mitre_techniques.iter().map(String::as_str).collect();
        let candidate_techniques: HashSet<&str> = self.extract_techniques(candidate);
        let technique_overlap = if query_techniques.is_empty() {
            0.0
        } else {
            let intersection = query_techniques.intersection(&candidate_techniques).count();
            intersection as f64 / query_techniques.len() as f64
        };
        let query_tags: HashSet<&str> = query.tags.iter().map(String::as_str).collect();
        let candidate_tags: HashSet<&str> = self.extract_tags(candidate);
        let tag_overlap = if query_tags.is_empty() {
            0.0
        } else {
            let intersection = query_tags.intersection(&candidate_tags).count();
            intersection as f64 / query_tags.len() as f64
        };
        0.6 * technique_overlap + 0.4 * tag_overlap
    }

    fn extract_techniques<'a>(&self, memory: &'a MemoryObject) -> HashSet<&'a str> {
        if let Some(techniques) = memory.context.get("techniques").and_then(|v| v.as_array()) {
            techniques.iter().filter_map(|v| v.as_str()).collect()
        } else {
            HashSet::new()
        }
    }

    fn extract_tags<'a>(&self, memory: &'a MemoryObject) -> HashSet<&'a str> {
        if let Some(tags) = memory.context.get("tags").and_then(|v| v.as_array()) {
            tags.iter().filter_map(|v| v.as_str()).collect()
        } else {
            HashSet::new()
        }
    }
}

impl Default for StructuralScorer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SemanticScorer;

impl SemanticScorer {
    pub fn new() -> Self {
        Self
    }

    pub fn score(&self, _query: &CanonicalIncident, _candidate: &MemoryObject) -> f64 {
        0.5
    }
}

impl Default for SemanticScorer {
    fn default() -> Self {
        Self::new()
    }
}
