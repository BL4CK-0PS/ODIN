# REST API

Base URL

/api/v1

---

POST

/incidents/upload

Description

Upload investigation logs.

Input

Multipart File

Output

IncidentID

---

GET

/incidents/{id}

Retrieve canonical incident.

---

POST

/incidents/search

Find similar investigations.

Body

Incident ID

TopK

---

GET

/incidents/{id}/timeline

Timeline

---

GET

/incidents/{id}/graph

Knowledge Graph

---

GET

/incidents/{id}/memory

Threat Memory

---

GET

/incidents/{id}/playbooks

Recommended Playbooks

---

POST

/incidents/{id}/feedback

Store analyst feedback.

---

GET

/system/health

Health Check

---

GET

/system/version

Version

---

Response Format

{
  success,
  data,
  metadata,
  errors
}
