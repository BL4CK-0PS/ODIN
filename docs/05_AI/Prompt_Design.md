# Prompt Design

## Principles

1. **Structured input** — always provide incident data as JSON not prose
2. **Explicit constraints** — temperature, max tokens, response format
3. **Few-shot examples** — every template includes 1–2 examples
4. **No instruction ambiguity** — use bullet points, not paragraphs
5. **Output schema** — use JSON mode / structured output whenever possible

## Narrative Generation Prompt

```
System: You are a DFIR report writer for ODIN.
Generate incident narratives that are factual, evidence-based, and concise.
Never speculate. Never add information not present in the input.
Use MITRE ATT&CK technique names and IDs.

Input:
{
  "title": "Spearphishing leading to Cobalt Strike",
  "severity": "Critical",
  "status": "Closed",
  "techniques": ["T1566.001", "T1059.001", "T1071.001"],
  "timeline": [
    {"time": "2026-06-15T08:23:00Z", "event": "Phishing email sent"},
    {"time": "2026-06-15T08:25:00Z", "event": "User opened attachment"}
  ],
  "observables": [
    {"type": "ip", "value": "5.6.7.8", "context": "C2 server"},
    {"type": "sha256", "value": "a1b2...", "context": "malicious payload"}
  ],
  "remediation": ["Isolate host", "Block C2 IP at firewall"]
}

Generate three sections:
1. EXECUTIVE_SUMMARY: 3-5 sentences for non-technical stakeholders
2. TECHNICAL_DETAILS: 5-10 sentences with specific observables and events
3. REMEDIATION_STEPS: Bulleted list of actions taken
```

## Similarity Explanation Prompt

```
System: You explain why two cybersecurity incidents are similar.
Compare their techniques, IOCs, and patterns.
Be specific about what matches.

Input:
{
  "query_incident": {
    "title": "...",
    "techniques": ["T1059.001", "T1566.001"],
    "iocs": {"ip": ["1.2.3.4"], "domain": ["evil.com"]}
  },
  "match_incident": {
    "title": "...",
    "techniques": ["T1059.001", "T1071.001"],
    "iocs": {"ip": ["1.2.3.4"], "domain": ["malware.net"]}
  },
  "similarity_score": 0.87,
  "score_breakdown": {
    "vector_similarity": 0.92,
    "technique_overlap": 0.50,
    "ioc_overlap": 0.33,
    "severity_match": 1.0
  }
}

Explain in 2-3 sentences:
- Which techniques overlap
- Which IOCs match
- What this suggests about the relationship
```

## MITRE Technique Suggestion Prompt

```
System: Suggest MITRE ATT&CK techniques based on evidence text.
Return up to 5 technique IDs with confidence scores.
Only suggest techniques with clear evidence support.

Input:
{
  "evidence_text": "PowerShell executed encoded command from remote server.
                    Scheduled task created for persistence.
                    Process injected into svchost.exe.",
  "context": "Initial access vector unknown"
}

Return JSON array:
[
  {"technique_id": "T1059.001", "confidence": 0.95, "reasoning": "PowerShell execution"},
  {"technique_id": "T1053.005", "confidence": 0.90, "reasoning": "Scheduled task creation"},
  {"technique_id": "T1055.001", "confidence": 0.85, "reasoning": "Process injection into svchost"}
]
```

## Template Versioning

- Every template has a `version` field in metadata
- Prompts are stored in `templates/{category}/{template_name}.md`
- Migration path: `v1 → v2` with date in changelog
- Current versions are in `docs/prompts/manifest.json`
