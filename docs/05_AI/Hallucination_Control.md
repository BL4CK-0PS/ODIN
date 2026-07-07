# Hallucination Control

## Problem

LLMs may generate text not supported by evidence. In DFIR, hallucinated IOCs, techniques, or remediation steps undermine trust and can cause real harm.

## Mitigation Layers

### Layer 1: Prompt Engineering
- System prompt explicitly forbids speculation
- "Only use information from the provided JSON input"
- Output schema enforced via JSON mode
- Few-shot examples show desired specificity

### Layer 2: Output Validation
```rust
struct ValidationResult {
    passed: bool,
    violations: Vec<Violation>,
}

enum Violation {
    UnknownIoc { value: String },
    UnknownTechnique { technique_id: String },
    ContradictsEvidence { claim: String, evidence_snippet: String },
    MissingRequiredSection { section: String },
}
```

Validation checks:
- All mentioned IOCs exist in incident data (regex match)
- All technique IDs are valid MITRE IDs
- Remediation steps don't contradict evidence
- No PII leakage (email, internal IPs, hostnames)

### Layer 3: Confidence Thresholds
| Output Type | Min Confidence | Action Below Threshold |
|-------------|---------------|------------------------|
| Executive summary | 0.7 | Flag for review |
| Technical details | 0.8 | Flag for review |
| Technique mapping | 0.9 | Require analyst confirmation |
| Remediation steps | 0.8 | Flag for review |
| Similarity explanation | 0.7 | Use rule-based fallback |

### Layer 4: Human Review
- Every narrative is presented as "draft" until analyst confirms
- Diff view: show generated vs. edited version
- Track acceptance rate per model for quality monitoring

## Monitoring

```rust
struct NarrativeQualityReport {
    model: String,
    total_generated: u64,
    accepted_without_edit: u64,
    accepted_with_edit: u64,
    rejected: u64,
    validation_failures: u64,
    avg_factual_score: f64,    // 0-100 from analyst feedback
    hallucination_incidents: Vec<HallucinationRecord>,
}
```

Tracked over time to detect model drift or prompt degradation.
