# Error Handling

## Philosophy

Every error must be:
1. **Captured** — never silently swallowed
2. **Logged** — with correlation ID and context
3. **Categorized** — client vs. server error
4. **Actionable** — hints for resolution where possible

## Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum OdinError {
    #[error("not found: {resource} {id}")]
    NotFound { resource: &'static str, id: String },

    #[error("validation error: {field}: {message}")]
    Validation { field: String, message: String, value: Option<String> },

    #[error("conflict: {message}")]
    Conflict { message: String },

    #[error("unauthorized: {message}")]
    Unauthorized { message: String },

    #[error("rate limited: retry after {retry_after}s")]
    RateLimited { retry_after: u64 },

    #[error("external service error: {service}: {message}")]
    ExternalService { service: String, message: String, status_code: u16 },

    #[error("internal error: {context}")]
    Internal { context: String, source: Option<Box<dyn std::error::Error + Send>> },

    #[error("LLM error: {message}")]
    Llm { message: String, provider: String, model: String },
}
```

## HTTP Mapping

| OdinError | HTTP Status |
|-----------|-------------|
| NotFound | 404 |
| Validation | 422 |
| Conflict | 409 |
| Unauthorized | 401 |
| RateLimited | 429 |
| ExternalService | 502 (or passthrough) |
| Internal | 500 |
| Llm | 503 |

## Error Response

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Title is required",
    "details": {
      "field": "title",
      "value": null
    },
    "request_id": "req_abc123"
  }
}
```

## Recovery Strategies

| Error | Recovery |
|-------|----------|
| Database connection lost | Retry with backoff (3 attempts), then circuit break |
| LLM timeout | Retry once, then return cached/fallback |
| External enrichment timeout | Skip enrichment, mark as pending | 
| Qdrant unavailable | Serve full-text search only, log and alert |
| Neo4j unavailable | Return entity-less results, log and alert |
