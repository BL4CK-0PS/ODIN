# High-Level Architecture

Version: 2.0

---

# Philosophy

ODIN is an **operating system for cyber knowledge**.

It does not process logs.

It processes **knowledge**.

Every log, alert, and investigation is transformed into structured intelligence that compounds over time.

---

# Core Principles

1. **Knowledge First** — Everything eventually becomes knowledge. Not logs. Not JSON. Not vectors. Knowledge.

2. **Evidence Never Lies** — Evidence is immutable. Everything else can evolve.

3. **Intelligence Is Derived** — Nothing "knows" anything. Everything derives intelligence from evidence.

4. **Memory Compounds** — Every investigation permanently increases organizational intelligence. Never overwrite. Only append.

5. **AI Never Owns Truth** — LLMs explain. Evidence decides.

---

# Architecture Layers

```
                    Existing Security Stack

Sysmon  Zeek  Suricata  Velociraptor  Wazuh  Elastic  Sentinel

              │
              ▼
═══════════════════════════════════════════════
           Observation Layer
═══════════════════════════════════════════════
Raw Logs → Normalize → Validate → Observations

              │
              ▼
═══════════════════════════════════════════════
         Knowledge Kernel (Core)
═══════════════════════════════════════════════
  ┌────────────────────────────────────────┐
  │  Canonical Incident  │  Evidence       │
  │  Entity              │  Relationship   │
  │  Memory Object                         │
  └────────────────────────────────────────┘
  Nothing outside the kernel modifies these.

              │
              ▼
═══════════════════════════════════════════════
              Knowledge Bus
═══════════════════════════════════════════════
  Domain Events: IncidentCreated, EvidenceAdded,
  MemoryIndexed, SimilarityComputed, ...

              │
     ┌────────┼────────┐
     ▼        ▼        ▼
════════ ═══════ ═══════════
Intelligence Retrieval Decision
  Engine     Engine    Engine
════════ ═══════ ═══════════
     │        │        │
     └────────┼────────┘
              │
              ▼
═══════════════════════════════════════════════
        Cyber Knowledge Fabric
═══════════════════════════════════════════════
  PostgreSQL  │  Qdrant  │  Neo4j (Enterprise)
  (Source of Truth)  (Vector Index)  (Graph Projection)

              │
              ▼
═══════════════════════════════════════════════
         Plugin Manager
═══════════════════════════════════════════════
  Parser  │  Storage  │  AI  │  Export  │  Search  │  Visualization

              │
              ▼
═══════════════════════════════════════════════
          Presentation Layer
═══════════════════════════════════════════════
  Web  │  API  │  CLI  │  Reports
```

---

# Core Business Capabilities

1. **Intelligence Engine** — Observations → Evidence → Knowledge
2. **Memory Engine** — Store, version, evolve, decay knowledge over time
3. **Retrieval Engine** — Hybrid similarity with context-aware ranking
4. **Decision Engine** — Evidence-backed recommendations with confidence propagation
5. **Policy Engine** — Governance gates on every operation

---

# Knowledge Kernel

The Kernel owns exactly five object types. Nothing outside can modify them.

```
Knowledge Kernel
├── Canonical Incident
├── Evidence
├── Entity
├── Relationship
└── Memory Object
```

Everything communicates through the Kernel. No subsystem directly mutates kernel state.

---

# Knowledge Bus

Services never call each other directly. They publish and subscribe to domain events.

```
IncidentCreated      → MemoryIndexed    → SimilarityComputed
EvidenceAdded        → GraphUpdated     → RecommendationGenerated
KnowledgeVersioned   → PolicyEvaluated  → NotificationSent
```

This decouples every subsystem and enables independent scaling, retry, and testing.

---

# Context Engine

Every investigation happens inside context. Similarity without context is weak.

```
Organization → Department → Environment → Assets → Threat Landscape → Incident
```

The same PowerShell command in a bank ≠ military network ≠ university. Context weights every similarity score.

---

# Intelligence Object Trait

Every domain object derives from one trait.

```rust
trait IntelligenceObject {
    fn id(&self) -> Uuid;
    fn provenance(&self) -> Provenance;
    fn confidence(&self) -> f32;
    fn trust(&self) -> f32;
    fn evidence(&self) -> Vec<EvidenceId>;
}
```

This gives every piece of knowledge: provenance, confidence, trust, and evidence links.

---

# Core vs. Enterprise

**ODIN Core (Hackathon MVP)**
- Knowledge Kernel
- Intelligence Engine
- Memory Engine
- Retrieval Engine
- Simple Decision Engine
- PostgreSQL + Qdrant
- In-memory graph projection
- Local LLM for explanation

**ODIN Enterprise (Roadmap)**
- Cyber Knowledge Fabric (Neo4j)
- Policy Engine
- Context Engine
- Knowledge Evolution
- Memory Decay
- Trust Propagation
- Domain Event Bus (Kafka/NATS)
- Multi-tenancy
- Cross-organization federation
