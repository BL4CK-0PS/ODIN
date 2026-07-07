# Logging

## Structured Logging

All services log in JSON format for machine parsing.

## Log Levels

| Level | When to Use |
|-------|-------------|
| ERROR | Service cannot function, data loss possible |
| WARN | Degraded operation, retry happening, non-critical failure |
| INFO | State changes (incident created, job completed) |
| DEBUG | Detailed operation info (enabled per-module) |
| TRACE | Full request/response bodies (dev only) |

## Required Fields

Every log entry must include:

```json
{
  "timestamp": "2026-06-15T08:23:00.123Z",
  "level": "INFO",
  "service": "odin-api",
  "request_id": "req_abc123",
  "message": "Incident created",
  "incident_id": "inc_01h2x3..."
}
```

`request_id` is generated at API gateway and propagated via tracing context.

## Tracing

Using `tracing` + `tracing-subscriber` for distributed tracing.

### Span Hierarchy

```
POST /api/v1/incidents
├── authenticate_request
├── validate_body
├── create_incident
│   ├── store_in_postgres
│   ├── extract_entities
│   └── enqueue_jobs
│       ├── enrich_evidence
│       └── generate_embedding
└── format_response
```

### Context Propagation

- HTTP headers: `x-request-id`, `x-span-id`
- Redis: `request_id` in job metadata
- LLM calls: `request_id` in API call headers

## Log Configuration

```toml
[logging]
format = "json"            # json | text
level = "info"             # Default log level
enable_tracing = true

[logging.overrides]
"odin_memory::embedder" = "debug"    # Per-module override
```

## Sensitive Data Redaction

- Never log: JWT tokens, API keys, passwords
- Redact: full IPs → first 3 octets only, emails → domain only
- Configurable regex-based redaction list
