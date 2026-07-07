# Component Architecture

# Components

## API Gateway

Responsibilities

- Upload logs
- Authentication
- Routing
- Validation

---

## Incident Reconstruction Service

Responsibilities

- Parse logs
- Normalize events
- Build timeline
- Extract entities
- Map MITRE

Outputs

Canonical Incident

---

## Memory Builder Service

Responsibilities

- Convert incident into reusable knowledge
- Generate relationships
- Store evidence
- Store lessons
- Build graph objects

---

## Threat Memory Service

Responsibilities

- Store incident embeddings
- Search historical incidents
- Rank similarity
- Explain similarity
- Generate investigation diff

---

## Graph Service

Responsibilities

- Query Neo4j
- Build relationships
- Return graph

---

## Search Service

Responsibilities

- Semantic Search
- Entity Search
- Investigation Search

---

## Frontend

Responsibilities

- Upload UI
- Timeline
- Graph
- Investigation Viewer
