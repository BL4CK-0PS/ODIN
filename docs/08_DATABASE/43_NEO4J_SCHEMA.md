# Neo4j Schema

---

# Node Labels

Incident

User

Host

Process

File

Registry

Network

IOC

Technique

Evidence

Playbook

Investigation

---

# Relationships

HOSTED_ON

EXECUTED

CREATED

MODIFIED

USES

CONNECTED_TO

OBSERVED_IN

MITIGATED_BY

RELATED_TO

SIMILAR_TO

REFERENCES

GENERATED

LEARNED_FROM

---

# Example

(:Incident)-[:OBSERVED_IN]->(:Process)

(:Process)-[:CONNECTED_TO]->(:Host)

(:Host)-[:USES]->(:Credential)

(:Incident)-[:MITIGATED_BY]->(:Playbook)

---

# Constraints

Incident ID unique

Entity UUID unique

Playbook ID unique
