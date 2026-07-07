# Narrative Generation

## Pipeline

```
[Incident Closed or Manual Trigger]
          │
          ▼
[Collect Structured Data]
  ├── incident metadata (title, severity, status)
  ├── techniques (MITRE IDs + names)
  ├── timeline (chronological events)
  ├── observables (IOCs with context)
  ├── evidence descriptions
  ├── analyst notes
  └── remediation actions
          │
          ▼
[Build Prompt Context]
  ├── truncate timeline to 50 most recent events
  ├── deduplicate observables
  ├── format as JSON
  └── select appropriate template
          │
          ▼
[LLM Generation]
  ├── provider: configured LLM
  ├── temperature: 0.3 (low for consistency)
  ├── max_tokens: 4096
  └── response_format: JSON
          │
          ▼
[Post-Processing]
  ├── validate JSON schema
  ├── PII scrub (emails, internal IPs if configured)
  ├── sanity check (no contradiction with evidence)
  └── strip markdown if plain text requested
          │
          ▼
[Store & Return]
  ├── save to incident.narrative (JSON)
  ├── cache for 5 minutes
  └── return to caller
```

## Output Schema

```rust
#[derive(Serialize, Deserialize)]
struct Narrative {
    version: String,            // "1.0"
    generated_at: DateTime<Utc>,
    model: String,              // "gpt-4o-mini"
    template_version: String,   // "narrative.v2"
    executive_summary: String,
    technical_details: String,
    remediation_steps: Vec<String>,
    // Optional: per-section sources for grounding
    sources: HashMap<String, Vec<SourceRef>>,
}

struct SourceRef {
    evidence_id: Uuid,
    evidence_type: String,
    snippet: String,
}
```

## Narrative Sections

| Section | Audience | Length | Style |
|---------|----------|--------|-------|
| Executive Summary | CISO/Exec | 3-5 sentences | Non-technical, impact-focused |
| Technical Details | SOC/DFIR | 5-10 sentences | Evidence-linked, precise |
| Timeline Summary | All | Bulleted events | Chronological |
| Remediation Steps | SOC/IT | Bulleted actions | Action-oriented |

## Quality Checks

| Check | Method | Action on Failure |
|-------|--------|-------------------|
| JSON valid | Schema validation | Retry with stricter system prompt |
| Contains required sections | Field presence check | Regenerate |
| No hallucinated IOCs | IOC match against evidence | Strip and flag for review |
| Factual consistency | Evidence cross-check | Flag for human review |
| PII leakage | Regex + pattern match | Redact and regenerate |
