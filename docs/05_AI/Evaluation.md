# Evaluation

## Narrative Quality Evaluation

### Automated Metrics
| Metric | Method | Target |
|--------|--------|--------|
| Factual consistency | Evidence cross-check | > 95% |
| Completeness | Required sections present | 100% |
| IOC hallucination rate | Regex match against source | < 1% |
| PII leakage rate | Regex detection | 0% |
| JSON schema compliance | Schema validation | 100% |
| Latency | End-to-end timing | < 10s P95 |

### Human Evaluation
Track via analyst feedback on each generated narrative:

```rust
struct NarrativeFeedback {
    narrative_id: Uuid,
    analyst_id: Uuid,
    factual_accuracy: u8,     // 1-5
    completeness: u8,         // 1-5
    clarity: u8,             // 1-5
    would_edit: bool,
    edits_made: Option<String>,
    submitted_at: DateTime<Utc>,
}
```

## Similarity Quality Evaluation

| Metric | Method | Target |
|--------|--------|--------|
| Precision@10 | % of top-10 results judged relevant | > 80% |
| Recall@50 | % of all relevant results in top-50 | > 70% |
| MAP@10 | Mean average precision | > 0.75 |
| NDCG@10 | Normalized discounted cumulative gain | > 0.80 |

Evaluation dataset: 100 labeled incident pairs per workspace.

## MITRE Mapping Accuracy

| Metric | Method | Target |
|--------|--------|--------|
| Precision | % of auto-mapped techniques correct | > 90% |
| Recall | % of manually-assigned techniques auto-detected | > 70% |
| F1 | Harmonic mean | > 0.78 |

## A/B Testing Framework

- Support for model comparison: same incident → two models
- Blind review: analysts rate without knowing which model
- Results tracked in `evaluation/` directory as JSONL

## Continuous Improvement

1. Collect feedback from analyst edits
2. Identify patterns in rejected narratives
3. Update prompts for common failure modes
4. Re-run evaluation suite
5. Version-bump prompt templates
