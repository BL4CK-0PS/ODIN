# Background Jobs

## Job Queue

Redis-backed job queue using `redis-rs` with custom worker pool.

## Job Types

```rust
enum JobType {
    EnrichEvidence { evidence_id: Uuid },
    GenerateEmbedding { incident_id: Uuid },
    GenerateNarrative { incident_id: Uuid },
    ExtractEntities { incident_id: Uuid },
    ReIndexIncident { incident_id: Uuid },
    BulkReIndex { workspace_id: Uuid },
    CleanupExpiredMemory,
    SendNotification { incident_id: Uuid, event: String },
}
```

## Worker Pool

```rust
struct WorkerPool {
    concurrency: usize,      // default: 4
    queue_name: String,      // default: "odin:jobs"
    poll_interval_ms: u64,   // default: 1000
    max_retries: u32,        // default: 3
}
```

## Job Lifecycle

```
Enqueued ──► Claimed ──► Running ──► Completed
                              │
                              ▼
                           Failed ──► Retry (if retries left)
                              │
                              ▼
                           Dead (no retries left)
```

## Retry Policy

| Job Type | Max Retries | Backoff | Timeout |
|----------|-------------|---------|---------|
| EnrichEvidence | 3 | Exponential (10s, 30s, 60s) | 30s |
| GenerateEmbedding | 2 | Exponential (5s, 15s) | 10s |
| GenerateNarrative | 2 | Exponential (10s, 30s) | 60s |
| ExtractEntities | 3 | Exponential (5s, 15s, 30s) | 20s |
| ReIndexIncident | 1 | Linear (30s) | 120s |
| CleanupExpiredMemory | 0 | N/A | 300s |

## Monitoring

- Job duration histograms (per type)
- Queue depth gauge
- Dead letter queue count alert (> 10)
- Worker saturation alert (> 80% busy)

## Graceful Shutdown

1. Signal received → stop polling
2. Wait for in-flight jobs to complete (max 30s)
3. Re-queue incomplete jobs
4. Shut down workers
