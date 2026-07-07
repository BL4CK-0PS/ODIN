# Similarity Scoring

## Detailed Scoring Pipeline

```
Query Incident (Q)
    │
    ▼
[Step 1: Embedding]
    │  Q_vector = embed(Q_text)
    │
    ▼
[Step 2: Qdrant ANN Search]
    │  Results: vector neighbors with cosine scores
    │  Pre-filter: technique overlap > 0 if selected
    │  ef: 50 (search width)
    │  Top-K: 50 (candidates for re-ranking)
    │
    ▼
[Step 3: Technique Jaccard]
    │  For each candidate C:
    │    tech_score = |Q.techniques ∩ C.techniques| / |Q.techniques ∪ C.techniques|
    │
    ▼
[Step 4: IOC Overlap]
    │  For each candidate C:
    │    ioc_score = |Q.iocs ∩ C.iocs| / max(|Q.iocs|, |C.iocs|)
    │
    ▼
[Step 5: Severity Match]
    │  sev_score = severity_match(Q.severity, C.severity)
    │
    ▼
[Step 6: Hybrid Score]
    │  total = 0.65*vector + 0.20*tech + 0.10*ioc + 0.05*sev
    │
    ▼
[Step 7: Re-rank & Return]
    │  Sort by total_score descending
    │  Return Top-10 with explanations
    │
    ▼
[Step 8: Explanation Generation]
    │  "Similar because: Same T1059.001 technique, 3 matching IOCs,
    │   and similar severity (Critical)"
```

## Scoring Example

| Candidate | Vector | Tech | IOC | Sev | Total | Interpretation |
|-----------|--------|------|-----|-----|-------|----------------|
| A | 0.92 | 0.80 | 0.33 | 1.0 | 0.84 | Same campaign |
| B | 0.85 | 0.50 | 0.00 | 1.0 | 0.68 | Related TTPs |
| C | 0.76 | 0.33 | 0.50 | 0.5 | 0.62 | Partial overlap |
| D | 0.45 | 0.00 | 0.00 | 0.0 | 0.29 | Low relevance |
