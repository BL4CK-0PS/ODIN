# Error Handling

---

Principles

Errors are typed.

Errors are recoverable.

Errors are logged.

Errors are traceable.

---

Categories

Validation

Parsing

Storage

Similarity

Graph

Reasoning

Internal

---

Example

ValidationError

Missing Event ID

↓

HTTP 400

---

StorageError

Database unavailable

↓

Retry

↓

HTTP 503

---

SimilarityError

No historical incidents

↓

Return empty result

↓

Do not fail request

---

ReasoningError

LLM unavailable

↓

Fallback

↓

Deterministic explanation

---

Logging

Every error contains

Timestamp

Request ID

Incident ID

Trace ID

Severity
