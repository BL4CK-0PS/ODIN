# Incident Model

## Schema

```rust
struct Incident {
    id: Uuid,
    title: String,
    description: Option<String>,
    severity: Severity,         // Critical, High, Medium, Low, Info
    status: IncidentStatus,     // Open, Triaging, Contained, Eradicated, Closed
    source: String,             // SIEM name, webhook source
    source_id: Option<String>,  // External alert ID
    mitre_techniques: Vec<TechniqueId>,
    tags: Vec<String>,
    assigned_to: Option<Uuid>,  // User ID
    workspace_id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    closed_at: Option<DateTime<Utc>>,
}

enum Severity { Critical, High, Medium, Low, Info }

enum IncidentStatus {
    Open,
    Triaging,
    Contained,
    Eradicated,
    Closed,
}
```

## Status Flow

```
Open ──► Triaging ──► Contained ──► Eradicated ──► Closed
  │          │            │              │
  └──────────┴────────────┴──────────────┘──► Closed (direct)
```

## Validation Rules

- Title required, max 256 chars
- Source required, max 64 chars
- At least one of: description, observables, or evidence required
- MITRE techniques must exist in ATT&CK matrix
