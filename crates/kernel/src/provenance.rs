use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub source: String,
    pub source_type: SourceType,
    pub collected_by: String,
    pub collected_at: chrono::DateTime<chrono::Utc>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    Log,
    NetworkCapture,
    FileSystem,
    MemoryDump,
    ThreatIntel,
    AnalystReport,
    ToolOutput,
    Other(String),
}

impl Provenance {
    pub fn new(source: String, source_type: SourceType, collected_by: String) -> Self {
        Self {
            source,
            source_type,
            collected_by,
            collected_at: chrono::Utc::now(),
            confidence: 1.0,
        }
    }
}
