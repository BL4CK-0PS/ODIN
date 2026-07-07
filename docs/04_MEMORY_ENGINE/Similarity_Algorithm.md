# Similarity Algorithm

## Overview

ODIN uses a **hybrid similarity** approach combining vector semantic similarity with technique/Jaccard overlap.

## Scoring Formula

```
total_score = 0.65 × vector_similarity
            + 0.20 × technique_overlap
            + 0.10 × ioc_overlap
            + 0.05 × severity_match
```

### Vector Similarity
- Cosine similarity between incident embeddings
- Range: [0, 1]

### Technique Overlap
```
technique_overlap = |A ∩ B| / |A ∪ B|  (Jaccard)
```
Where A and B are sets of MITRE technique IDs.

### IOC Overlap
```
ioc_overlap = |IOC_A ∩ IOC_B| / max(|IOC_A|, |IOC_B|)
```
Normalized to prevent large-IOC incidents from dominating.

### Severity Match
```
severity_match = 1.0 if severity == query_severity
                 0.5 if adjacent (Critical↔High, High↔Medium, etc.)
                 0.0 otherwise
```

## Thresholds

| Score Range | Interpretation |
|-------------|---------------|
| > 0.85 | Very similar — likely same campaign |
| 0.70–0.85 | Similar — related TTPs or IOCs |
| 0.50–0.70 | Somewhat similar — partial overlap |
| < 0.50 | Low similarity |

## Optimization

- Pre-compute and cache technique overlap for frequently compared incidents
- Use Qdrant's full-text filter for technique pre-filtering before vector search
- Top-K results (default: 10) re-ranked by hybrid score
