# Relationships

## Entity Relationship Model

ODIN models relationships as **typed, directed edges** in Neo4j.

## Core Relationship Types

| Relationship | Source → Target | Description |
|-------------|----------------|-------------|
| `RESOLVES_TO` | Domain → IP | DNS resolution |
| `CONNECTS_TO` | IP → IP | Network connection |
| `DOWNLOADS_FROM` | Process → URL | File download |
| `EXECUTED_BY` | Process → Process | Parent-child process |
| `ACCESSED` | User → Host | User login |
| `OWNS` | ThreatActor → Campaign | Attribution |
| `USES` | Campaign → Technique | TTP association |
| `OBSERVED_IN` | IOC → Incident | IOC appears in incident |
| `ASSOCIATED_WITH` | Incident → Incident | Similar incident link |
| `ENRICHED_BY` | Entity → EnrichmentData | External enrichment |
| `MITIGATES` | DetectionRule → Technique | Coverage mapping |
| `TRIGGERED_BY` | Incident → DetectionRule | Alert origin |

## Relationship Properties

```rust
struct Relationship {
    id: Uuid,
    relationship_type: String,
    source_id: Uuid,
    target_id: Uuid,
    properties: HashMap<String, Value>,
    confidence: Option<f64>,     // 0.0 to 1.0
    first_seen: Option<DateTime<Utc>>,
    last_seen: Option<DateTime<Utc>>,
    source: String,              // "auto", "analyst", "import"
}
```

## Example Graph Query

```cypher
// Find all C2 infrastructure used in similar incidents
MATCH (i1:Incident {id: $incident_id})
MATCH (i1)-[:SIMILAR_TO]->(i2:Incident)
MATCH (i2)-[:OBSERVED_IN]->(ip:IpAddress)
WHERE ip.entity_type = 'IpAddress'
  AND i2.mitre_techniques CONTAINS 'T1071.001'
RETURN ip.value, count(i2) as incident_count
ORDER BY incident_count DESC
```
