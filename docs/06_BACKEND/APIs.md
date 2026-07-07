# APIs

## Base URL

`/api/v1`

## Authentication

All endpoints except `/auth/*` and `/health` require `Authorization: Bearer <jwt>`.

## Endpoints

### Incidents

```
POST   /incidents                    # Create incident
GET    /incidents                    # List incidents (paginated)
GET    /incidents/{id}               # Get incident detail
PUT    /incidents/{id}               # Update incident
PATCH  /incidents/{id}/status        # Transition status
DELETE /incidents/{id}               # Delete incident
POST   /incidents/{id}/merge         # Merge with another incident
```

**Query parameters for GET /incidents:**
- `status` — filter by status
- `severity` — filter by severity
- `technique` — filter by MITRE technique ID
- `search` — full-text search
- `tag` — filter by tag
- `page`, `per_page` — pagination (default: 1, 20)

### Evidence

```
POST   /incidents/{id}/evidence          # Add evidence
GET    /incidents/{id}/evidence          # List evidence
GET    /evidence/{id}                    # Get evidence detail
DELETE /evidence/{id}                    # Delete evidence
POST   /evidence/{id}/enrich            # Trigger enrichment
```

### Memory / Similarity

```
POST   /incidents/{id}/similar           # Find similar incidents
GET    /memory/search?q=<query>          # Search memory by text
POST   /memory/reindex/{id}             # Re-index single incident
POST   /memory/reindex                  # Re-index all (admin)
```

### Narrative

```
POST   /incidents/{id}/narrative          # Generate narrative
GET    /incidents/{id}/narrative          # Get stored narrative
PUT    /incidents/{id}/narrative          # Update narrative (analyst edit)
POST   /incidents/{id}/narrative/export   # Export as PDF/MD/JSON
```

### Graph

```
POST   /graph/entities                   # Create entity
GET    /graph/entities?type=<type>       # List entities
GET    /graph/entities/{id}              # Get entity
POST   /graph/relationships              # Create relationship
GET    /graph/relationships              # Query relationships
POST   /graph/path                       # Find path between entities
```

### Search

```
GET    /search?q=<query>&type=<type>     # Hybrid search
```

### Playbooks (Future)

```
POST   /playbooks                        # Create playbook
GET    /playbooks                        # List playbooks
POST   /playbooks/{id}/execute           # Execute playbook
GET    /playbooks/{id}/executions        # List executions
```

### Authentication

```
POST   /auth/login                       # Get JWT
POST   /auth/refresh                     # Refresh JWT
POST   /auth/logout                      # Invalidate token
```

### Health

```
GET    /health                           # Service health
GET    /health/ready                     # Readiness check
```

## Response Format

```json
{
  "data": { ... },
  "meta": {
    "page": 1,
    "per_page": 20,
    "total": 142,
    "total_pages": 8
  }
}
```

## Error Format

```json
{
  "error": {
    "code": "INCIDENT_NOT_FOUND",
    "message": "Incident with id 'abc-123' not found",
    "details": {}
  }
}
```

## Rate Limits

| Tier | Requests/min |
|------|-------------|
| Authenticated | 1000 |
| Unauthenticated | 60 |
| Ingestion webhook | 5000 |
