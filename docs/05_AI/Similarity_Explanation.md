# Similarity Explanation

## Why Explainability Matters

ODIN's similarity engine returns matches by vector distance, but analysts need to know *why* two incidents are related to trust and act on the recommendation.

## Explanation Types

### 1. Technique-Based
```
"Both incidents involve T1059.001 (PowerShell) and T1566.001 (Spearphishing Attachment)."
```

### 2. IOC-Based
```
"Both incidents share C2 infrastructure: 5.6.7.8 appears in both."
```

### 3. Pattern-Based
```
"Both incidents follow the same pattern: phishing → macro → PowerShell → Cobalt Strike."
```

### 4. Hybrid
```
"3 matching techniques (T1059.001, T1071.001, T1055.001) and 2 overlapping IOCs suggest same threat actor."
```

## Explanation Generation Flow

```
[Hybrid Score Components]
    │
    ▼
[Rule-Based Template Selection]
    │
    ├── If technique_overlap > 0.5 → technique template
    ├── If ioc_overlap > 0.3 → IOC template
    ├── If both high → hybrid template
    └── Else → general similarity template
    │
    ▼
[Optional: LLM Refinement]
    │  If confidence < threshold → use LLM to generate
    │  natural language from structured explanation
    │
    ▼
[Return to UI]
    │  Display: score bar + explanation text + matched items
```

## UI Display

```
Similarity: 87% — Very Similar
──────────────────────────────────────
  Techniques (50% overlap)
    T1059.001  ■■■■■■■□□□  PowerShell
    T1566.001  ■■■■■■■■■■  Spearphishing
    T1071.001  ■■■■■□□□□□  Web Protocols

  IOCs (2 matching)
    ● 5.6.7.8 (C2 IP)
    ● evil.example.com (C2 domain)

  Past remediation applied:
    ✓ Host isolation
    ✓ C2 block at firewall
    ✓ Credential reset
```

## Explanation Storage

```rust
struct SimilarityExplanation {
    query_incident_id: Uuid,
    match_incident_id: Uuid,
    score: f64,
    explanation_text: String,
    matched_techniques: Vec<TechniqueMatch>,
    matched_iocs: Vec<IocMatch>,
    generated_by: ExplanationMethod, // RuleBased | LLM
    generated_at: DateTime<Utc>,
}

struct TechniqueMatch {
    technique_id: String,
    name: String,
    overlap_count: u32,
}

struct IocMatch {
    value: String,
    ioc_type: String,
    context: Option<String>,
}
```
