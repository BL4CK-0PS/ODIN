# MITRE Mapping

## Purpose

Map evidence and observables to MITRE ATT&CK techniques automatically, reducing manual classification work for analysts.

## Mapping Methods

### Method 1: Direct Rule Mapping (Priority: High)
- Sigma/YARA rule → technique mapping from rule metadata
- EDR alert → technique mapping (e.g., Defender for Endpoint event IDs)

```
Example:
  Sigma rule: posh_encoded_command.yml
  Mapping: T1059.001 (Command and Scripting Interpreter: PowerShell)
  Confidence: 1.0
```

### Method 2: IOC-Based Mapping (Priority: Medium)
| Observable Type | Likely Techniques |
|----------------|-------------------|
| Phishing email | T1566.001, T1566.002 |
| Base64-encoded string | T1059.001, T1027 |
| C2 domain (known-bad) | T1071.001, T1572 |
| Scheduled task | T1053.005 |
| Registry run key | T1547.001 |

### Method 3: LLM-Assisted Mapping (Priority: Low)
- For ambiguous evidence, use LLM to suggest techniques
- Always return confidence score
- Always require human confirmation for confidence < 0.8

```
Input: "Process created schtasks.exe to create 'Updater' task"
Output: T1053.005, confidence 0.95
```

## Mapping Schema

```rust
struct TechniqueMapping {
    incident_id: Uuid,
    technique_id: String,
    method: MappingMethod,  // DirectRule | IocBased | LLM | Manual
    confidence: f64,         // 0.0 to 1.0
    source_evidence_id: Option<Uuid>,
    source_evidence_text: Option<String>,
    mapped_by: Uuid,         // system user or analyst
    mapped_at: DateTime<Utc>,
    confirmed: bool,         // analyst has accepted/rejected
}

enum MappingMethod {
    DirectRule,
    IocBased,
    LLM,
    Manual,
}
```

## MITRE Data Source

ODIN ships with a local copy of MITRE ATT&CK (ENTERPRISE) as embedded JSON:

```
data/mitre/enterprise-attack.json
```

- Loaded at startup into memory
- Refreshed via `odin-mitre refresh` CLI command
- Supports filtering by platform (Windows, Linux, macOS)
