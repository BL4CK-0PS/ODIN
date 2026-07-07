# ODIN

**Operational Defense Intelligence Network**

*Every investigation strengthens tomorrow's defense.*

---

## 📌 Overview

ODIN is an **Institutional Cyber Memory** platform that transforms individual incident investigations into lasting, reusable organizational intelligence. Rather than generating static reports, ODIN builds a compounding knowledge base where each investigation enriches the collective understanding of your security landscape.

At its core, ODIN is an **operating system for cyber knowledge** — not a linear pipeline, but a resilient kernel that ingests raw observations and continuously synthesizes them into actionable memory. The platform empowers security teams to investigate faster, make better decisions, and reduce mean time to resolution (MTTR) through intelligent memory recall and contextual awareness.

---

## ✨ Key Features

| Feature | Description |
|---------|-------------|
| **Incident Reconstruction** | Upload logs, auto-generate chronological timelines, extract relevant entities (IPs, domains, users, files), and map events to the MITRE ATT&CK framework with context-aware confidence scoring. |
| **Memory Engine** | Persist investigations as structured knowledge objects with full versioning, temporal weighting, and lifecycle tracking — ensuring that insights grow in value over time. |
| **Hybrid Similarity Search** | Combine structural, semantic, and context-ranked retrieval to find the most relevant past investigations, TTPs, and relationships, even from partial or ambiguous queries. |
| **Investigation Diff** | Compare current and historical investigations at a granular level — view changes in entity associations, timeline events, and confidence scores to understand how understanding evolves. |
| **Cyber Knowledge Fabric** | Visualize relationships across entities, incidents, techniques, and indicators of compromise (IoCs) in an interactive graph interface that reveals hidden connections. |
| **Explainable AI** | Every AI-driven recommendation is accompanied by confidence propagation paths, provenance chains, and policy-gated rationales — so analysts know *why* a recommendation was made. |
| **Local AI** | Powered by Ollama with support for Qwen 3 (reasoning) and Nomic Embed (retrieval). All processing occurs on-premise — **no data leaves your infrastructure**. |

---

## 🏛 Architecture

### High-Level Data Flow

```mermaid
flowchart LR
    subgraph Input["Input Layer"]
        LOGS[("Logs\n(Syslog/Events)")]
        ALERTS[("Alerts\n(SIEM/IDS)")]
        IOCS[("IOCs\n(Threat Intel)")]
    end

    subgraph Kernel["Knowledge Kernel"]
        direction TB
        CI["Canonical Incident"]
        CE["Canonical Evidence"]
        ENT["Canonical Entity"]
        REL["Canonical Relationship"]
        MO["Memory Object"]
        CI --- CE --- ENT --- REL --- MO
    end

    subgraph Engines["Core Engines"]
        IE["Intelligence Engine\nExtract & Enrich"]
        ME["Memory Engine\nStore & Version"]
        RE["Retrieval Engine\nSearch & Rank"]
        DE["Decision Engine\nRecommend & Predict"]
        PG["Policy Gate\nGovern & Filter"]
    end

    subgraph Output["Output Layer"]
        UI["Analyst Interface"]
        API["External APIs"]
        REP["Reports"]
    end

    LOGS --> IE
    ALERTS --> IE
    IOCS --> IE
    IE --> CI
    IE --> CE
    IE --> ENT
    IE --> REL
    CI --> ME
    CE --> ME
    ENT --> ME
    REL --> ME
    ME --> MO
    MO --> RE
    RE --> DE
    DE --> PG
    PG --> UI
    PG --> API
    PG --> REP

    classDef input fill:#e1f5fe,stroke:#01579b
    classDef kernel fill:#f3e5f5,stroke:#4a148c
    classDef engine fill:#fff3e0,stroke:#e65100
    classDef output fill:#e8f5e9,stroke:#1b5e20
    class LOGS,ALERTS,IOCS input
    class CI,CE,ENT,REL,MO kernel
    class IE,ME,RE,DE,PG engine
    class UI,API,REP output
```

---

### Five Core Engines — Detailed Flow

```mermaid
flowchart TD
    subgraph IE["🧠 Intelligence Engine"]
        EXTRACT["Entity Extraction\n(IP, Domain, User, File, Process)"]
        ENRICH["Context Enrichment\n(WHOIS, GeoIP, Threat Feeds)"]
        MAP["MITRE ATT&CK Mapping\n(Tactics, Techniques, Procedures)"]
        TIMELINE["Timeline Generation\n(Chronological Event Ordering)"]
    end

    subgraph ME["💾 Memory Engine"]
        STORE["Store Investigation\n(Canonical Format)"]
        VERSION["Version Control\n(Snapshot per Change)"]
        WEIGHT["Temporal Weighting\n(Recency & Frequency Decay)"]
        LIFECYCLE["Lifecycle Management\n(Active → Archived → Purged)"]
    end

    subgraph RE["🔍 Retrieval Engine"]
        SEMANTIC["Semantic Search\n(Embedding Similarity)"]
        STRUCTURAL["Structural Search\n(Graph Pattern Matching)"]
        CONTEXT["Context Ranking\n(Recency, Relevance, Confidence)"]
        HYBRID["Hybrid Fusion\n(Ranked Result Merging)"]
    end

    subgraph DE["🎯 Decision Engine"]
        RECOMMEND["Case Recommendation\n(Similar Past Incidents)"]
        PREDICT["Next-Step Prediction\n(Investigation Pathways)"]
        CONFIDENCE["Confidence Propagation\n(Provenance Chains)"]
    end

    subgraph PG["🛡️ Policy Gate"]
        AUTH["Authentication & RBAC"]
        AUDIT["Audit Logging"]
        FILTER["Data Filtering\n(Need-to-Know Basis)"]
        COMPLIANCE["Compliance Enforcement\n(GDPR, HIPAA, etc.)"]
    end

    IE --> ME
    ME --> RE
    RE --> DE
    DE --> PG

    classDef engine fill:#fff3e0,stroke:#e65100,stroke-width:2px
    classDef step fill:#fafafa,stroke:#9e9e9e,stroke-width:1px
    class IE,ME,RE,DE,PG engine
    class EXTRACT,ENRICH,MAP,TIMELINE,STORE,VERSION,WEIGHT,LIFECYCLE,SEMANTIC,STRUCTURAL,CONTEXT,HYBRID,RECOMMEND,PREDICT,CONFIDENCE,AUTH,AUDIT,FILTER,COMPLIANCE step
```

---

### End-to-End Investigation Pipeline

```mermaid
flowchart TD
    START([🔵 Investigation Starts]) --> UPLOAD["Upload Logs & Artifacts"]
    UPLOAD --> PARSE["Parse & Normalize\n(Structured Event Extraction)"]
    PARSE --> EXTRACT["Extract Entities\n(IPs, Domains, Users, Hashes)"]
    EXTRACT --> TIMELINE["Build Timeline\n(Event Sequencing)"]
    TIMELINE --> MAP["Map to MITRE ATT&CK\n(TTP Identification)"]
    MAP --> QUERY["Query Memory Engine\n(Find Similar Cases)"]
    
    QUERY --> |Similar Found| COMPARE["Investigation Diff\n(Compare with Past Cases)"]
    QUERY --> |No Similar| NEW["Create New Memory Object"]
    
    COMPARE --> RECOMMEND["AI Recommendations\n(Next Steps, TTPs, IoCs)"]
    NEW --> RECOMMEND
    
    RECOMMEND --> POLICY["Policy Gate Review\n(Compliance & Access Control)"]
    POLICY --> |Approved| PRESENT["Present to Analyst"]
    POLICY --> |Blocked| REJECT["Reject / Redact Output"]
    
    PRESENT --> ANALYST{"Analyst Decision"}
    ANALYST --> |Accept| STORE["Store Investigation\n(Versioned Memory Object)"]
    ANALYST --> |Modify| UPDATE["Update Investigation\n(Create New Version)"]
    ANALYST --> |Reject| DISCARD["Discard / Archive"]
    
    UPDATE --> STORE
    STORE --> FEEDBACK["Feedback Loop\n(Reinforcement Learning)"]
    FEEDBACK --> END([🟢 Investigation Complete])
    DISCARD --> END
    REJECT --> END

    classDef startend fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px
    classDef process fill:#e3f2fd,stroke:#1565c0,stroke-width:1px
    classDef decision fill:#fff3e0,stroke:#e65100,stroke-width:2px
    classDef storage fill:#f3e5f5,stroke:#4a148c,stroke-width:1px
    class START,END startend
    class UPLOAD,PARSE,EXTRACT,TIMELINE,MAP,QUERY,COMPARE,RECOMMEND,PRESENT startend
    class ANALYST decision
    class STORE,UPDATE,DISCARD,FEEDBACK storage
```

---

### Knowledge Object Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Draft: Investigation Initiated
    Draft --> UnderReview: Analyst Submission
    UnderReview --> Active: Approved
    UnderReview --> Draft: Revisions Requested
    Active --> Enhanced: New Evidence Added
    Enhanced --> Active: Re-verified
    Active --> Deprecated: Outdated / Superseded
    Deprecated --> Archived: Retention Period Expired
    Archived --> Purged: Permanent Deletion
    Purged --> [*]
    
    note right of Active
        Available for:
        - Similarity Search
        - Investigation Diff
        - Knowledge Fabric
    end note
    
    note right of Enhanced
        Version Increment
        Temporal Weight Recalculated
    end note
    
    note right of Deprecated
        Still Searchable
        Weight Decayed
    end note
```

---

### Similarity Search — Hybrid Retrieval Pipeline

```mermaid
flowchart LR
    subgraph Query["🔎 Query Processing"]
        Q_TEXT["Text Query"]
        Q_STRUCT["Structural Query"]
        Q_CTX["Context Filters\n(Time, TTP, Severity)"]
    end

    subgraph Semantic["🧬 Semantic Path"]
        EMBED["Embedding Generation\n(Nomic Embed)"]
        VS["Vector Search\n(Qdrant)"]
        SCORE1["Semantic Score\n(0.0 - 1.0)"]
    end

    subgraph Structural["🔗 Structural Path"]
        PATTERN["Graph Pattern\n(Cypher Query)"]
        GS["Graph Search\n(Neo4j)"]
        SCORE2["Structural Score\n(0.0 - 1.0)"]
    end

    subgraph Fusion["⚡ Fusion & Ranking"]
        NORMALIZE["Score Normalization"]
        WEIGHT["Weighted Fusion\n(α·Semantic + β·Structural + γ·Context)"]
        RERANK["Contextual Reranking\n(Recency, Analyst Feedback)"]
        TOPK["Top-K Results"]
    end

    Q_TEXT --> EMBED
    Q_STRUCT --> PATTERN
    Q_CTX --> WEIGHT
    
    EMBED --> VS
    VS --> SCORE1
    
    PATTERN --> GS
    GS --> SCORE2
    
    SCORE1 --> NORMALIZE
    SCORE2 --> NORMALIZE
    Q_CTX --> WEIGHT
    
    NORMALIZE --> WEIGHT
    WEIGHT --> RERANK
    RERANK --> TOPK

    classDef query fill:#e1f5fe,stroke:#01579b
    classDef semantic fill:#f3e5f5,stroke:#4a148c
    classDef structural fill:#fff3e0,stroke:#e65100
    classDef fusion fill:#e8f5e9,stroke:#1b5e20
    class Q_TEXT,Q_STRUCT,Q_CTX query
    class EMBED,VS,SCORE1 semantic
    class PATTERN,GS,SCORE2 structural
    class NORMALIZE,WEIGHT,RERANK,TOPK fusion
```

---

### Development Workflow

```mermaid
gitGraph
    commit id: "Initial commit"
    branch develop
    checkout develop
    commit id: "Setup project structure"
    
    branch feature/odin-core
    checkout feature/odin-core
    commit id: "Add knowledge kernel"
    commit id: "Implement memory engine"
    
    checkout develop
    branch feature/odin-frontend
    commit id: "Setup Next.js app"
    commit id: "Add investigation UI"
    
    checkout feature/odin-core
    commit id: "Add similarity search"
    commit id: "Implement diff engine"
    
    checkout develop
    merge feature/odin-core id: "Merge core engine"
    
    checkout feature/odin-frontend
    commit id: "Add visualization"
    commit id: "Integrate with API"
    
    checkout develop
    merge feature/odin-frontend id: "Merge frontend"
    
    branch feature/odin-infra
    checkout feature/odin-infra
    commit id: "Docker compose setup"
    commit id: "Add CI/CD pipeline"
    
    checkout develop
    merge feature/odin-infra id: "Merge infrastructure"
    
    checkout main
    merge develop id: "Release v1.0.0"
    commit id: "Tag: v1.0.0"
```

---

### Team Collaboration & Decision Flow

```mermaid
flowchart TD
    subgraph Input["📥 Input"]
        REQ["Feature Request\n/ Bug Report"]
        IDEA["New Idea"]
        CHG["Change Proposal"]
    end

    subgraph Process["⚙️ Process"]
        direction LR
        TRIAGE["Triage\n(Project Lead)"]
        ASSIGN["Assign Owner\n(By Domain)"]
        
        subgraph Domains["Domain Owners"]
            KISH["Kishanth\nArchitecture/Backend/Frontend"]
            DHAR["Dharshan\nInfrastructure/DevOps/Database"]
            SHREY["Shreyanth\nSecurity/Threat Intel/Crypto"]
        end
        
        DESIGN["Design Document\n(Owner + Reviewers)"]
        APPROVAL{"Approval\nGate"}
        IMPLEMENT["Implementation\n(Feature Branch)"]
        REVIEW["Peer Review\n(Mandatory)"]
        TEST["Integration Testing\n(CI/CD)"]
        MERGE["Merge to Main"]
    end

    subgraph Output["📤 Output"]
        DEPLOY["Deployment\n(Staging → Production)"]
        RELEASE["Release\n(All Team Sign-off)"]
        DOC["Documentation Update"]
    end

    REQ --> TRIAGE
    IDEA --> TRIAGE
    CHG --> TRIAGE
    
    TRIAGE --> ASSIGN
    ASSIGN --> DESIGN
    DESIGN --> Domains
    Domains --> APPROVAL
    
    APPROVAL --> |Approved| IMPLEMENT
    APPROVAL --> |Rejected| REQ
    
    IMPLEMENT --> REVIEW
    REVIEW --> TEST
    TEST --> MERGE
    MERGE --> DEPLOY
    DEPLOY --> RELEASE
    RELEASE --> DOC
    
    classDef input fill:#e1f5fe,stroke:#01579b
    classDef process fill:#fff3e0,stroke:#e65100
    classDef domain fill:#f3e5f5,stroke:#4a148c
    classDef decision fill:#ffebee,stroke:#c62828
    classDef output fill:#e8f5e9,stroke:#1b5e20
    class REQ,IDEA,CHG input
    class TRIAGE,ASSIGN,DESIGN,IMPLEMENT,REVIEW,TEST,MERGE process
    class KISH,DHAR,SHREY domain
    class APPROVAL decision
    class DEPLOY,RELEASE,DOC output
```

---

### Deployment Architecture

```mermaid
flowchart TB
    subgraph External["🌐 External"]
        USER["👤 Analyst / Admin"]
        FEEDS["📡 Threat Intel Feeds"]
    end

    subgraph CDN["🚀 CDN / Load Balancer"]
        LB["Load Balancer"]
    end

    subgraph Frontend["🖥️ Frontend Layer"]
        NEXT["Next.js App\n(Server-Side Rendering)"]
        STATIC["Static Assets\n(Images, CSS, JS)"]
    end

    subgraph Backend["⚙️ Backend Layer"]
        API["Rust API Server\n(Axum)"]
        WORKERS["Background Workers\n(Async Processing)"]
        CACHE["Redis Cache\n(Query Results, Sessions)"]
    end

    subgraph Storage["💾 Storage Layer"]
        PG[("PostgreSQL\n(Structured Data)")]
        QD[("Qdrant\n(Vector Store)")]
        NEO[("Neo4j\n(Graph Store)")]
        S3[("S3 / MinIO\n(Artifacts, Logs)")]
    end

    subgraph AI["🤖 AI Layer"]
        OLLAMA["Ollama\n(Qwen 3, Nomic Embed)"]
    end

    subgraph Observability["📊 Observability"]
        PROM["Prometheus\n(Metrics)"]
        GRAF["Grafana\n(Dashboards)"]
        LOKI["Loki\n(Logs)"]
    end

    USER --> LB
    FEEDS --> API
    LB --> NEXT
    NEXT --> STATIC
    NEXT --> API
    
    API --> CACHE
    API --> WORKERS
    API --> PG
    API --> QD
    API --> NEO
    API --> S3
    API --> OLLAMA
    
    WORKERS --> PG
    WORKERS --> QD
    WORKERS --> NEO
    WORKERS --> S3
    
    API --> PROM
    WORKERS --> PROM
    PROM --> GRAF
    API --> LOKI
    WORKERS --> LOKI

    classDef external fill:#e1f5fe,stroke:#01579b
    classDef cdn fill:#fff3e0,stroke:#e65100
    classDef frontend fill:#f3e5f5,stroke:#4a148c
    classDef backend fill:#e8f5e9,stroke:#1b5e20
    classDef storage fill:#fff9c4,stroke:#f57f17
    classDef ai fill:#fce4ec,stroke:#c62828
    classDef obs fill:#e0f7fa,stroke:#006064
    class USER,FEEDS external
    class LB cdn
    class NEXT,STATIC frontend
    class API,WORKERS,CACHE backend
    class PG,QD,NEO,S3 storage
    class OLLAMA ai
    class PROM,GRAF,LOKI obs
```

---

### Security & Compliance Flow

```mermaid
flowchart TD
    INPUT["🔐 Incoming Request\n(Investigation Query / Data Access)"]
    INPUT --> AUTH["Authentication\n(JWT / OAuth2 / API Key)"]
    
    AUTH --> |Valid| RBAC{"RBAC Check\n(Role-Based Access Control)"}
    AUTH --> |Invalid| REJECT1["❌ Reject: Unauthenticated"]
    
    RBAC --> |Authorized| AUDIT1["📝 Audit Log: Access Granted"]
    RBAC --> |Unauthorized| REJECT2["❌ Reject: Insufficient Permissions"]
    
    AUDIT1 --> DECRYPT{"Data Encrypted?"}
    DECRYPT --> |Yes| DECRYPT_DATA["Decrypt\n(AES-256 / Quantum-Safe)"]
    DECRYPT --> |No| PASS
    
    DECRYPT_DATA --> PASS["Pass Data to Policy Gate"]
    PASS --> POLICY{"Policy Evaluation"}
    
    POLICY --> P1["Need-to-Know Check"]
    POLICY --> P2["Data Classification Check\n(Public/Internal/Confidential/Secret)"]
    POLICY --> P3["Compliance Check\n(GDPR, HIPAA, FedRAMP)"]
    
    P1 --> P1_RESULT{"Pass?"}
    P1_RESULT --> |No| REDACT["Redact Sensitive Fields"]
    P1_RESULT --> |Yes| P2
    
    P2 --> P2_RESULT{"Pass?"}
    P2_RESULT --> |No| REDACT
    P2_RESULT --> |Yes| P3
    
    P3 --> P3_RESULT{"Pass?"}
    P3_RESULT --> |No| REJECT3["❌ Reject: Compliance Violation"]
    P3_RESULT --> |Yes| ALLOW["✅ Allow Access"]
    
    REDACT --> ALLOW
    ALLOW --> RESPONSE["📤 Return Response"]
    RESPONSE --> AUDIT2["📝 Audit Log: Response Sent"]
    
    REJECT1 --> AUDIT_ERR["📝 Audit Log: Error"]
    REJECT2 --> AUDIT_ERR
    REJECT3 --> AUDIT_ERR
    
    AUDIT_ERR --> END([🔚 End])
    AUDIT2 --> END

    classDef input fill:#e1f5fe,stroke:#01579b
    classDef decision fill:#fff3e0,stroke:#e65100,stroke-width:2px
    classDef reject fill:#ffebee,stroke:#c62828,stroke-width:2px
    classDef allow fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px
    classDef process fill:#f3e5f5,stroke:#4a148c
    classDef audit fill:#fff9c4,stroke:#f57f17
    class INPUT input
    class RBAC,DECRYPT,POLICY decision
    class REJECT1,REJECT2,REJECT3 reject
    class ALLOW allow
    class AUTH,PASS,P1,P2,P3,REDACT process
    class AUDIT1,AUDIT2,AUDIT_ERR audit
```

---

## 🧱 Tech Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| **Backend** | Rust (Axum) | High-performance, memory-safe API and business logic |
| **Frontend** | Next.js | Reactive, accessible UI with server-side rendering |
| **Structured Store** | PostgreSQL | Transactional storage for investigations, users, and metadata |
| **Vector Store** | Qdrant | Embedding-based similarity search for semantic retrieval |
| **Graph Store** | Neo4j (Enterprise) / In-memory (Core) | Relationship storage and graph traversal for knowledge fabric |
| **Cache** | Redis | Query caching, session management, rate limiting |
| **AI** | Ollama (Qwen 3, Nomic Embed) | Local LLM for reasoning, summarization, and embedding generation |
| **Observability** | Prometheus + Grafana + Loki | Metrics, dashboards, and log aggregation |

---

## 👥 Team

ODIN is built by a multidisciplinary team with deep expertise in cybersecurity, software engineering, and emerging technologies.

---

### Kishanth R
**Project Lead · Full Stack Engineer · Cybersecurity Engineer**

**Primary Responsibilities**
- Product vision, architecture, and technical leadership
- Rust backend development (Axum) and Next.js frontend
- API design, Threat Memory Engine, Similarity Engine, Incident Reconstruction
- AI integration, system integration, and final demo/presentation delivery

**Domain Ownership:** Intelligence Engine, Memory Engine, Retrieval Engine, Decision Engine, Frontend, Overall System Architecture

---

### Dharshan
**DevOps Engineer · Cybersecurity Engineer**

**Primary Responsibilities**
- Infrastructure architecture, Docker containerization, and CI/CD pipeline automation
- Deployment automation and orchestration
- PostgreSQL management, Qdrant & Neo4j deployment and tuning
- Monitoring, observability, backend testing, and infrastructure security hardening

**Domain Ownership:** Infrastructure Layer, Deployment, Storage Layer (PostgreSQL, Qdrant, Neo4j), Monitoring, Security Operations

---

### Shreyanth
**Cybersecurity Engineer · Cryptography & Quantum Security Researcher**

**Primary Responsibilities**
- Threat intelligence, MITRE ATT&CK mapping, and threat modeling
- Detection engineering and signature development
- Cryptographic design and quantum-safe security research
- Security validation, investigation methodology, dataset validation, and security documentation

**Domain Ownership:** Threat Intelligence, Security Domain, Investigation Models, Detection Logic, Security Research, Cryptographic Components

---

### 🤝 Collaboration Model

| Area | Owner | Reviewer(s) |
|------|-------|-------------|
| System Architecture | Kishanth | Entire Team |
| Backend Development | Kishanth | Dharshan |
| Frontend Development | Kishanth | Dharshan |
| Infrastructure & DevOps | Dharshan | Kishanth |
| Database Management | Dharshan | Kishanth |
| Threat Intelligence | Shreyanth | Kishanth |
| MITRE ATT&CK Mapping | Shreyanth | Kishanth |
| Cryptography & Security | Shreyanth | Entire Team |
| Documentation | Entire Team | Project Lead |
| Final Integration | Kishanth | Entire Team |

---

### 🧭 Decision Ownership

| Area | Decision Owner | Reviewers |
|------|---------------|-----------|
| Product Vision & Strategy | Kishanth | — |
| Architecture Decisions | Kishanth | Entire Team |
| Infrastructure Decisions | Dharshan | Kishanth |
| Security & Cryptographic Decisions | Shreyanth | Kishanth |
| Final Release Approval | — | All Team Members (unanimous) |

---

### 🔄 Development Workflow

```mermaid
gitGraph
    commit id: "Initial commit"
    branch develop
    checkout develop
    commit id: "Setup project"
    
    branch feature/odin-core
    checkout feature/odin-core
    commit id: "Add knowledge kernel"
    commit id: "Implement memory engine"
    
    checkout develop
    branch feature/odin-frontend
    commit id: "Setup Next.js"
    commit id: "Add investigation UI"
    
    checkout feature/odin-core
    commit id: "Add similarity search"
    commit id: "Implement diff engine"
    
    checkout develop
    merge feature/odin-core id: "Merge core engine"
    
    checkout feature/odin-frontend
    commit id: "Add visualization"
    commit id: "Integrate API"
    
    checkout develop
    merge feature/odin-frontend id: "Merge frontend"
    
    branch feature/odin-infra
    checkout feature/odin-infra
    commit id: "Docker setup"
    commit id: "Add CI/CD"
    
    checkout develop
    merge feature/odin-infra id: "Merge infra"
    
    checkout main
    merge develop id: "Release v1.0.0"
    tag: "v1.0.0"
```

---

### 💬 Communication

| Channel | Frequency | Purpose |
|---------|-----------|---------|
| Daily Sync | Daily (standup) | Progress updates, blockers, integration status |
| Team Discussions | As needed | Architecture changes, security decisions, roadmap planning |
| Critical Changes | Before implementation | API changes, schema migrations, infrastructure updates |

---

### 🧠 Guiding Principle

> *Every team member owns a domain, but the product belongs to the entire team.*

Individual ownership drives velocity; shared responsibility ensures quality, reliability, and collective success.

---

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable)
- Node.js (v18+)
- Docker & Docker Compose

### Quick Start

```bash
# Clone the repository
git clone https://github.com/your-org/odin.git
cd odin

# Start infrastructure services
docker compose up -d postgres qdrant ollama redis

# Start the Rust backend
cargo run

# Start the Next.js frontend
cd apps/web && npm run dev
```

Once running, access the platform at `http://localhost:3000`.

For detailed setup instructions, troubleshooting, and environment configuration, refer to the [Developer Guide](docs/15_ENGINEERING/76_DEVELOPER_GUIDE.md).

---

## 📚 Documentation

All project documentation is organized in the `docs/` directory with a structured numbering system for easy navigation.

| Section | Contents |
|---------|----------|
| **00_OVERVIEW** | Vision, problem statement, competitive landscape |
| **01_PRODUCT** | User personas, user stories, requirements, success metrics |
| **02_ARCHITECTURE** | High-level design, component interactions, data flow, deployment models |
| **03_DOMAIN** | Canonical incident, entity, evidence, and relationship models |
| **04_THREAT_MEMORY** | Memory engine architecture, similarity search, diffing, object lifecycle |
| **05_AI** | AI architecture, reasoning pipelines, narrative generation, evaluation frameworks |
| **06_BACKEND** | Rust workspace structure, crate architecture, API endpoints, service layer |
| **07_FRONTEND** | UI architecture, design system, page structure, component library |
| **08_DATABASE** | PostgreSQL schemas, Neo4j graph models, Qdrant collection design, versioning strategy |
| **09_SECURITY** | Authentication, RBAC, audit logging, encryption, evidence integrity |
| **10_DEPLOYMENT** | Docker configuration, CI/CD pipelines, monitoring, environment variables |
| **11_API** | Domain event definitions, REST API reference, plugin system |
| **12_TESTING** | Unit, integration, end-to-end, and performance testing strategies |
| **13_DEMO** | Demo script, dataset preparation, judge FAQ, presentation guide |
| **14_BUSINESS** | Market analysis, business model, go-to-market strategy, product roadmap |
| **15_ENGINEERING** | Developer guide, coding standards, architectural decision records (ADRs) |
| **16_OPERATIONS** | Runbook, incident response procedures, release management, glossary, references |
| **17_APPENDIX** | OpenAPI specification, datasets, backlog, implementation milestones |

---

## 📄 License

ODIN is released under the **Apache License 2.0**. See the `LICENSE` file for full terms.

---

## 🙏 Acknowledgements

ODIN builds on the work of many open-source projects and standards, including:

- [MITRE ATT&CK®](https://attack.mitre.org/) — For the industry-standard framework for describing adversary behavior.
- [Ollama](https://ollama.com/) — For enabling local, private AI inference.
- [Qdrant](https://qdrant.tech/) — For high-performance vector similarity search.
- [Neo4j](https://neo4j.com/) — For graph database capabilities.
- [Redis](https://redis.io/) — For high-performance caching.
- The Rust and Next.js communities — for their incredible ecosystems.

---

## 📬 Contact

For questions, collaboration inquiries, or security disclosures, please open an issue on GitHub or contact the project lead directly.

---

**ODIN — Turning every investigation into institutional memory.**
