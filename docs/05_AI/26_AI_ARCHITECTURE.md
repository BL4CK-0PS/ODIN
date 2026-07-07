# AI Architecture

Version: 1.0

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

---

# AI Responsibilities

AI MAY

✓ Generate attack narratives

✓ Explain similarities

✓ Summarize investigations

✓ Normalize analyst notes

✓ Generate recommendations

AI MUST NOT

✗ Invent evidence

✗ Create fake IOCs

✗ Modify timeline

✗ Change MITRE mappings without evidence

✗ Alter investigation history

---

# AI Pipeline

Raw Logs

↓

Rule Engine

↓

Canonical Incident

↓

Knowledge Extraction

↓

Threat Memory

↓

Reasoning Engine

↓

LLM

↓

Validated Response

---

# Models

Reasoning

Qwen 3

Embeddings

Nomic Embed

Future

Fine-tuned Security LLM

---

# Hallucination Policy

Every AI response must reference:

Evidence

Canonical Incident

Historical Memory

No unsupported statements allowed.
