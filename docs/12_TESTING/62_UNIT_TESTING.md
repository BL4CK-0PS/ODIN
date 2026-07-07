# Unit Testing

---

# Purpose

Validate individual business components.

---

# Components

Canonical Incident

Memory Builder

Similarity Engine

Diff Engine

Reasoning Engine

Timeline Builder

---

# Rules

Every public function must have tests.

Business rules must never rely on external services.

Mocks must replace databases.

---

# Coverage Goals

Business Logic

95%

Utilities

90%

Storage

80%

API

80%

---

# Example

Test

MemoryBuilder::create()

Given

Canonical Incident

Expect

Memory Object

Containing

- Entities

- Timeline

- Evidence

- Lessons
