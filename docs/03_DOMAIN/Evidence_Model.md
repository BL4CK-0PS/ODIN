# Evidence Model

## Evidence Types

```rust
enum EvidenceType {
    Observable(ObservableType),  // IP, domain, hash, URL, email
    File,                        // Uploaded file (PCAP, memory dump, etc.)
    Artifact,                    // Forensic artifact (registry, event log, etc.)
    Log,                         // Log entry
    Screenshot,                  // Image capture
    Note,                        // Analyst free-text note
}
```

## Evidence Schema

```rust
struct Evidence {
    id: Uuid,
    incident_id: Uuid,
    evidence_type: EvidenceType,
    title: String,
    description: Option<String>,
    source: String,              // How it was obtained
    timestamp: DateTime<Utc>,    // When the evidence was created/collected
    file_path: Option<String>,   // Path in object store
    file_size: Option<i64>,
    file_hash: Option<String>,   // SHA-256
    mime_type: Option<String>,
    metadata: HashMap<String, Value>,
    created_by: Option<Uuid>,    // Analyst who added it
    created_at: DateTime<Utc>,
}
```

## Evidence Lifecycle

1. **Ingested** — added to incident during creation or analyst action
2. **Enriched** — automatic enrichment (VT, DNS, WHOIS)
3. **Analyzed** — analyst reviews and annotates
4. **Linked** — connected to entities in knowledge graph
5. **Archived** — preserved with closed incident
