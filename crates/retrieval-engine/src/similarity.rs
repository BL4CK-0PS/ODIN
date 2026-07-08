use odin_kernel::{CanonicalIncident, MemoryObject};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};

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

    pub fn score(&self, query: &CanonicalIncident, candidate: &MemoryObject) -> f64 {
        let query_text = format!("{} {}", query.title, query.description);
        let query_tokens = self.tokenize(&query_text);
        if query_tokens.is_empty() {
            return 0.5;
        }

        let candidate_summary = &candidate.summary;
        let candidate_context = candidate.context.get("text_description")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let candidate_text = format!("{} {}", candidate_summary, candidate_context);
        let candidate_tokens = self.tokenize(&candidate_text);
        if candidate_tokens.is_empty() {
            return 0.3;
        }

        let query_freq = self.term_frequency(&query_tokens);
        let candidate_freq = self.term_frequency(&candidate_tokens);

        let all_terms: HashSet<&str> = query_freq.keys().chain(candidate_freq.keys()).copied().collect();
        let epsilon = 1e-10;
        let dot: f64 = all_terms.iter()
            .map(|t| query_freq.get(t).copied().unwrap_or(epsilon) * candidate_freq.get(t).copied().unwrap_or(epsilon))
            .sum();
        let q_mag: f64 = query_freq.values().map(|v| v * v).sum::<f64>().sqrt();
        let c_mag: f64 = candidate_freq.values().map(|v| v * v).sum::<f64>().sqrt();

        if q_mag < epsilon || c_mag < epsilon {
            let jaccard = self.jaccard_similarity(&query_tokens, &candidate_tokens);
            return jaccard;
        }

        let cosine = dot / (q_mag * c_mag);
        let jaccard = self.jaccard_similarity(&query_tokens, &candidate_tokens);
        0.6 * cosine + 0.4 * jaccard
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        text.split(|c: char| !c.is_alphanumeric() && c != '\'' && c != '-')
            .map(|w| w.trim().to_lowercase())
            .filter(|w| w.len() >= 3 && !STOP_WORDS.iter().any(|s| *s == w.as_str()))
            .collect()
    }

    fn term_frequency<'a>(&self, tokens: &'a [String]) -> HashMap<&'a str, f64> {
        let mut freq: HashMap<&str, f64> = HashMap::new();
        let total = tokens.len() as f64;
        if total == 0.0 { return freq; }
        for t in tokens {
            *freq.entry(t.as_str()).or_insert(0.0) += 1.0;
        }
        for v in freq.values_mut() {
            *v /= total;
        }
        freq
    }

    fn jaccard_similarity(&self, a: &[String], b: &[String]) -> f64 {
        let set_a: HashSet<&str> = a.iter().map(String::as_str).collect();
        let set_b: HashSet<&str> = b.iter().map(String::as_str).collect();
        let intersection = set_a.intersection(&set_b).count();
        let union = set_a.union(&set_b).count();
        if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
    }
}

impl Default for SemanticScorer {
    fn default() -> Self {
        Self::new()
    }
}

const STOP_WORDS: &[&str] = &[
    "the", "and", "for", "are", "but", "not", "you", "all", "can", "had",
    "her", "was", "one", "our", "out", "has", "have", "been", "some", "them",
    "than", "that", "this", "very", "just", "with", "will", "each", "make",
    "like", "from", "they", "been", "said", "what", "when", "where", "which",
    "their", "there", "would", "about", "into", "over", "such", "also",
];
