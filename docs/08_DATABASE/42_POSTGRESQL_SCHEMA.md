# PostgreSQL Schema

---

# Tables

incidents

entities

evidence

timeline_events

playbooks

lessons

users

audit_logs

attachments

---

# incidents

incident_id

title

severity

status

created_at

updated_at

analyst

confidence

summary

---

# entities

entity_id

incident_id

entity_type

entity_name

confidence

metadata

---

# evidence

evidence_id

incident_id

source

timestamp

description

confidence

reference

---

# timeline_events

event_id

incident_id

timestamp

event_type

description

---

# playbooks

playbook_id

title

description

version

---

# lessons

lesson_id

incident_id

lesson

severity

---

# Indexes

incident_id

entity_type

timestamp

severity

confidence
