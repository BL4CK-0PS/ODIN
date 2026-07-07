# Service Architecture

---

Upload Service

↓

Parser Service

↓

Reconstruction Service

↓

Memory Builder

↓

Storage Service

↓

Similarity Service

↓

Reasoning Service

↓

Presentation

---

Responsibilities

Upload

Receive logs.

---

Parser

Normalize events.

---

Reconstruction

Create Canonical Incident.

---

Memory

Store knowledge.

---

Similarity

Find historical investigations.

---

Reasoning

Generate explanation.

---

Presentation

Return analyst-friendly response.

---

Communication

Services communicate using traits.

No direct database coupling.
