# Narrative Engine

## Purpose

Generate a readable incident story.

---

# Input

Canonical Incident

---

# Output

Human-readable investigation narrative.

---

# Example

The attacker executed PowerShell with an encoded command.

The script created a Registry Run key to establish persistence.

Credential dumping activity was detected shortly afterwards.

The attacker attempted lateral movement before being contained.

---

# Rules

Narratives must be chronological.

Narratives must reference evidence.

Narratives cannot invent events.

Narratives must remain concise.

---

# Narrative Structure

Introduction

↓

Initial Access

↓

Execution

↓

Persistence

↓

Privilege Escalation

↓

Credential Access

↓

Lateral Movement

↓

Collection

↓

Impact

↓

Summary
