# Canonical Incident Model

## Purpose

The Canonical Incident is the central data model of ODIN.

Every log source, investigation, and similarity search is transformed into this unified representation.

No raw logs are stored inside the Canonical Incident.

Instead, it stores extracted intelligence.

---

# Structure

Incident

├── Metadata

├── Timeline

├── Entities

├── Evidence

├── Techniques

├── Indicators

├── Lessons Learned

├── Playbooks

├── Investigation Notes

└── Confidence

---

# Metadata

- Incident ID
- Title
- Description
- Severity
- Status
- Source
- Created Time
- Updated Time
- Analyst

---

# Timeline

Ordered list of security events.

Each event includes

- Timestamp
- Event Type
- Source
- Description
- Related Entities

---

# Entities

All extracted security objects.

Examples

- Users
- Hosts
- Processes
- Files
- Registry Keys
- Services
- Scheduled Tasks
- Domains
- IP Addresses

---

# Evidence

Every fact supporting conclusions.

Evidence is immutable.

---

# ATT&CK Techniques

Mapped techniques.

Each technique contains

- Technique ID
- Technique Name
- Confidence
- Supporting Evidence

---

# Indicators

Extracted IOCs

- IP

- Domain

- Hash

- URL

- Email

- Registry Key

- File

---

# Lessons Learned

Analyst observations.

Reusable recommendations.

---

# Playbooks

Investigation playbooks that successfully resolved this incident.

---

# Confidence

Overall investigation confidence score.

Range

0-100
