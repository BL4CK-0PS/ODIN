# Reasoning Engine

## Purpose

Transform structured investigation data into analyst-quality reasoning.

---

# Inputs

Canonical Incident

Threat Memory

Knowledge Graph

MITRE Mapping

Evidence

---

# Workflow

Incident

↓

Evidence Validation

↓

Threat Memory Search

↓

Reasoning

↓

Recommendation

↓

Explanation

---

# Outputs

Investigation Summary

Likely Attack Chain

Historical Similarities

Playbook Suggestions

Evidence References

---

# Rules

Reasoning never modifies evidence.

Reasoning only interprets evidence.

Evidence always wins.

---

# Example

Input

PowerShell

Registry Run Key

Credential Dump

Output

Likely malicious persistence.

Reason:

PowerShell execution combined with Registry Run persistence has matched 14 previous incidents.

Confidence

91%

Supporting Evidence

Sysmon Event 1

Sysmon Event 13

Registry Modification
