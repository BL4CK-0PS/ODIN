# Similarity Engine

## Purpose

Find previous investigations that resemble a new incident.

---

# Inputs

Canonical Incident

Historical Memory Objects

---

# Stage 1

Structural Similarity

Compare

- MITRE Techniques
- Entities
- Processes
- Registry Keys
- Network Activity
- IOCs

---

# Stage 2

Semantic Similarity

Embedding comparison

Top K Retrieval

---

# Stage 3

Hybrid Ranking

Overall Score

=

40% Structural

+

40% Semantic

+

20% Investigation Context

---

# Outputs

Similarity Score

Historical Incident

Reasons

Supporting Evidence

Recommended Playbooks

Lessons Learned

---

# Explainability

Every match must answer

Why?

Example

92%

Matched because

✓ Same T1059

✓ Same PowerShell

✓ Same Registry Key

✓ Same Credential Access

---

# Confidence

Low

0-40

Medium

41-70

High

71-100
