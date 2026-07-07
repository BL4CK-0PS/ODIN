# Embedding Strategy

## Model Selection

### Primary Model: `all-MiniLM-L6-v2`
- **Dimensions:** 384
- **Performance:** Fast inference (~5ms on CPU)
- **Quality:** Good semantic understanding for security text
- **Trade-off:** Chosen over larger models (e.g., `all-mpnet-base-v2`, 768d) for speed

### Alternative Models (Configurable)
| Model | Dims | Quality | Speed | Use Case |
|-------|------|---------|-------|----------|
| all-MiniLM-L6-v2 | 384 | Good | Fast | Default |
| all-mpnet-base-v2 | 768 | Better | Medium | Higher accuracy need |
| intfloat/e5-mistral-7b-instruct | 4096 | Best | Slow | Research/premium tier |

## Embedding Content Strategy

### What Gets Embedded

Each incident produces a single combined embedding from:

```
embed_text = f"""
Title: {title}
Description: {description[:500]}
Techniques: {', '.join(technique_names)}
Tactics: {', '.join(tactic_names)}
IOCs: {', '.join(important_iocs[:20])}
Observable Types: {', '.join(observable_types)}
Tags: {', '.join(tags)}
"""
```

### Field Weights (for hybrid scoring, not embeddings)
| Field | Weight | Notes |
|-------|--------|-------|
| TTPs | 0.35 | Most discriminative |
| IOC values | 0.25 | Specific indicators |
| Description | 0.20 | Free-text context |
| Title | 0.10 | Summary signal |
| Tags | 0.10 | Analyst categorization |

## Regeneration Strategy

- Re-embed on: title/description change, technique re-mapping, tag changes
- Batch re-embed: nightly for stale incidents (>24h since last embed)
- Full re-index: on embedding model change (triggered manually)
