# Investigation Diff Engine

## Purpose

Similarity alone is not useful.

Analysts need differences.

---

# Inputs

Current Investigation

Historical Investigation

---

# Compare

Timeline

Entities

Processes

Files

Registry

IOCs

Techniques

Playbooks

Evidence

---

# Output

Similarities

Differences

Recommendations

---

# Example

Historical

PowerShell

Registry Run Key

Credential Dump

T1059

T1003

---

Current

PowerShell

Registry Run Key

Credential Dump

DLL Injection

T1059

T1003

T1055

---

Result

Similar

✓ PowerShell

✓ Registry

✓ Credential Dump

Different

• DLL Injection

• New Process Tree

Recommendation

Reuse Playbook IR-07

Add DLL investigation step.

---

# Goals

Reduce investigation time.

Improve analyst confidence.

Prevent missed differences.
