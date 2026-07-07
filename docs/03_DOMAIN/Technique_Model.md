# Technique Model

## Schema

```rust
struct Technique {
    id: TechniqueId,             // e.g., "T1059.001"
    name: String,                // e.g., "Command and Scripting Interpreter: PowerShell"
    tactic: Tactic,              // e.g., Execution
    platform: Vec<Platform>,     // e.g., Windows, Linux
    description: String,
    detection: Option<String>,
    mitigation: Option<String>,
    sigma_rules: Vec<SigmaRuleId>,
    observed_count: i64,         // Across all incidents in workspace
    last_observed: Option<DateTime<Utc>>,
}

enum Tactic {
    Reconnaissance,
    ResourceDevelopment,
    InitialAccess,
    Execution,
    Persistence,
    PrivilegeEscalation,
    DefenseEvasion,
    CredentialAccess,
    Discovery,
    LateralMovement,
    Collection,
    CommandAndControl,
    Exfiltration,
    Impact,
}
```

## Technique Mapping

Techniques are derived from evidence via:

1. **Direct mapping** — Sigma rule matches alert → technique
2. **IOC-based** — Observable type + context → technique
3. **Analyst assignment** — Manual technique tagging
4. **ML inference** — Future: predict techniques from evidence patterns

## Coverage Tracking

```rust
struct TechniqueCoverage {
    technique_id: TechniqueId,
    total_incidents: i64,
    detected_incidents: i64,     // Had detection rule firing
    manual_assignments: i64,     // Analyst identified post-hoc
    gap_score: f64,              // (total - detected) / total
    sigma_rule_count: i64,
    recommended_priority: Priority,
}
```
