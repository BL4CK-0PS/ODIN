# Domain Events

Version: 1.0

---

# Philosophy

ODIN follows Event Driven Architecture.

Business events drive the system.

Services never directly coordinate business workflows.

Instead, they publish and consume events.

---

# Event Lifecycle

Incident Uploaded

↓

Incident Parsed

↓

Canonical Incident Created

↓

Memory Created

↓

Similarity Search Completed

↓

Investigation Completed

---

# Events

## IncidentUploaded

Payload

- IncidentID
- UploadTime
- Source

---

## IncidentParsed

Payload

- IncidentID
- Timeline
- Entities
- Evidence

---

## CanonicalIncidentCreated

Payload

- CanonicalIncident

---

## MemoryCreated

Payload

- MemoryID
- Version

---

## SimilaritySearchCompleted

Payload

- IncidentID
- Similar Incidents
- Confidence

---

## InvestigationCompleted

Payload

- IncidentID
- Lessons
- Playbooks
