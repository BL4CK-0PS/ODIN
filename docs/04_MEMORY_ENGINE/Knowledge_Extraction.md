# Knowledge Extraction

## Extraction Sources

| Source | Extractable Entities |
|--------|---------------------|
| Alert title/description | Techniques, tactics, severity indicators |
| IOC values | IPs, domains, hashes, emails, URLs |
| Log entries | Hostnames, user names, process names, file paths |
| Analyst notes | Techniques, attacker hypotheses, campaign names |
| Enrichment results | ASN, registrar, VT relationships |
| EDR data | Process trees, parent/child relationships |

## Extraction Methods

### 1. Pattern-Based (Deterministic)
- Regex patterns for IP, domain, hash, URL, email
- File path patterns (/etc/, C:\, \\) 
- Registry key patterns
- Process name patterns (powershell.exe, cmd.exe)

### 2. MITRE-Based (Rule-Driven)
- Technique-specific IOC signatures
- Sigma rule context → technique mapping
- Observable type → likely technique(s)

### 3. LLM-Assisted (Configurable)
- Extract campaign names from analyst notes
- Identify threat actor mentions
- Infer relationships not explicitly stated
- Flagged for human review before graph insertion

## Quality Control

- Confidence threshold for auto-extraction: > 0.8
- Below 0.8: flagged as "suggested" requiring analyst confirmation
- Deduplication: normalized values compared before insertion
- Batch extraction runs post-ingestion (async, non-blocking)
