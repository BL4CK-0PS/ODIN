# AI Architecture

Version: 2.0

---

# Philosophy

AI is not the product.

Institutional Cyber Memory is the product.

AI exists to assist analysts, not replace them.

---

# Design Principles

- Deterministic first
- AI second
- Explainability first
- Human verification
- Local-first deployment
- Confidence propagation
- Policy-gated output

---

# AI Responsibilities

AI MAY

✓ Generate attack narratives

✓ Explain similarities

✓ Summarize investigations

✓ Normalize analyst notes

✓ Generate recommendations

AI MUST NOT

✗ Determine facts

✗ Invent evidence

✗ Create fake IOCs

✗ Modify timeline

✗ Change MITRE mappings without evidence

✗ Alter investigation history

---

# Intelligence Engine Pipeline

Observations

↓

Evidence Validation

↓

Knowledge Kernel

↓

Intelligence Engine

├── Deterministic Reasoning (rules + evidence matching)

├── Context Engine (organizational weighting)

├── LLM Enhancement (readability only)

└── Confidence Propagation (formula-based)

↓

Decision Engine

├── Recommendation (what to do)

├── Confidence (derived from evidence trust)

├── Evidence Links (provenance chain)

└── Policy Gate (allowed by governance?)

↓

Validated Output

---

# Decision Engine

Currently: Reasoning → Output

Missing: Reasoning → Decision → Recommendation → Confidence → Evidence → Action

```
Step 1: Gather evidence
Step 2: Query Memory Engine for similar cases
Step 3: Deterministic rules form hypothesis
Step 4: LLM generates readable explanation
Step 5: Confidence computed from evidence trust scores
Step 6: Policy engine validates
Step 7: Decision presented to analyst
```

---

# Confidence Propagation

Every recommendation carries a derived confidence, not a guess.

```
Recommendation: 91%

Derived From:
  Evidence A (trust: 0.94)
  Evidence B (trust: 0.88)
  Evidence C (trust: 0.97)

Formula: geometric_mean(0.94, 0.88, 0.97) = 0.91
```

Confidence is always traceable to source evidence.

---

# Trust Score

Every object in the system has trust.

```
Evidence:   100  (immutable source truth)
Entity:     97   (derived from evidence)
Relationship: 93 (derived from entities)
Memory:     91   (derived from relationships)
Recommendation: 87 (derived from memory + context)
```

Trust decays with derivation distance. This makes every number explainable.

---

# Policy Engine

Enterprise governance gates on all AI output.

```
Policies:
  ✗ Never recommend with confidence < 0.70
  ✗ Never expose restricted evidence
  ✗ Never summarize classified incidents
  ✓ Allow override with manager approval
```

Everything passes through policy before reaching the analyst.

---

# IntelligenceObject Trait

Every knowledge object shares this interface.

```rust
trait IntelligenceObject {
    fn id(&self) -> Uuid;
    fn provenance(&self) -> Provenance;
    fn confidence(&self) -> f32;
    fn trust(&self) -> f32;
    fn evidence(&self) -> Vec<EvidenceId>;
}
```

This unifies behavior across incidents, evidence, entities, and memory.

---

# Knowledge Provenance

Every piece of knowledge answers: where did this come from?

```
Knowledge
  ↓
Evidence
  ↓
Event
  ↓
Parser
  ↓
Original Log
  ↓
Timestamp
```

Nothing is anonymous. Every recommendation is traceable to source.

---

# Models

Reasoning: Qwen 3
Embeddings: Nomic Embed
Future: Fine-tuned Security LLM (knowledge-grounded)

---

# Hallucination Policy

- AI never determines facts
- Every response must reference evidence, Canonical Incident, or historical memory
- Confidence must be propagated from source, never invented
- Policy engine validates before delivery
