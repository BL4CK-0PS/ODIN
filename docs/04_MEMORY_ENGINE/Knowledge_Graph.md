# Knowledge Graph

## Graph Schema (Neo4j)

### Node Labels

```
(:Incident {id, title, severity, status, created_at})
(:Entity {id, entity_type, value, normalized_value, first_seen, last_seen})
(:Technique {id, name, tactic, platform})
(:ThreatActor {id, name, aliases})
(:Campaign {id, name, first_seen, last_seen})
(:DetectionRule {id, name, rule_type, source})
(:Workspace {id, name})
(:EnrichmentSource {id, name, type})
```

### Indexes

```cypher
CREATE INDEX entity_value_idx FOR (e:Entity) ON (e.normalized_value);
CREATE INDEX entity_type_idx FOR (e:Entity) ON (e.entity_type);
CREATE INDEX incident_created_idx FOR (i:Incident) ON (i.created_at);
CREATE INDEX technique_id_idx FOR (t:Technique) ON (t.id);
```

### Constraints

```cypher
CREATE CONSTRAINT unique_entity FOR (e:Entity) REQUIRE (e.normalized_value, e.entity_type, e.workspace_id) IS UNIQUE;
CREATE CONSTRAINT unique_incident FOR (i:Incident) REQUIRE i.id IS UNIQUE;
CREATE CONSTRAINT unique_technique FOR (t:Technique) REQUIRE t.id IS UNIQUE;
```

## Typical Queries

```cypher
// All entities connected to an incident
MATCH (i:Incident {id: $incident_id})-[:OBSERVED_IN]-(e:Entity)
RETURN e.entity_type, e.value, e.first_seen

// Path between two entities
MATCH path = shortestPath(
  (a:Entity {value: $source})-[:CONNECTS_TO|RESOLVES_TO|DOWNLOADS_FROM*]-(b:Entity {value: $target})
)
RETURN path

// Recurring TTPs across incidents
MATCH (i:Incident)-[:MAPS_TO]->(t:Technique)
RETURN t.id, t.name, count(i) as frequency
ORDER BY frequency DESC
```
