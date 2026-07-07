# Explainability Engine

## Purpose

Every recommendation must answer one question:

Why?

---

# Inputs

Similarity Result

Reasoning Output

Canonical Incident

Evidence

---

# Outputs

Explanation

Confidence

Supporting Evidence

Differences

Recommendations

---

# Example

Similarity

92%

Explanation

Matched because

✓ Same PowerShell

✓ Same Registry Key

✓ Same ATT&CK T1059

✓ Same Credential Dump

Differences

• New DLL Injection

Evidence

Sysmon Event 1

Sysmon Event 13

Registry Key HKCU\Run

---

# Confidence

0-40

Low

41-70

Medium

71-100

High

---

# Principles

No black-box AI.

Everything must be explainable.

Analysts must verify conclusions.
