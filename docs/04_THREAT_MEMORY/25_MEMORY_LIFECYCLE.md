# Memory Lifecycle

## Stage 1

Incident Created

↓

Canonical Incident

---

## Stage 2

Memory Builder

↓

Normalize

↓

Extract Knowledge

↓

Generate Summary

↓

Create Relationships

---

## Stage 3

Memory Storage

PostgreSQL

Neo4j

Qdrant

---

## Stage 4

Memory Retrieval

Similarity Search

↓

Historical Match

↓

Explanation

↓

Diff

---

## Stage 5

Analyst Feedback

Useful

Not Useful

False Match

---

## Stage 6

Memory Improvement

Adjust ranking

Update metadata

Improve retrieval

---

# Lifecycle Rules

Evidence is immutable.

Canonical Incident is immutable.

Memory versions are append-only.

Analyst feedback never overwrites history.

---

# Long-Term Goal

Every investigation increases the organization's collective cybersecurity knowledge.

Knowledge compounds over time.

ODIN becomes smarter because the organization becomes smarter.
