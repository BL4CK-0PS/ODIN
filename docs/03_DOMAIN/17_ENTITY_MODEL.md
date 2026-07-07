# Entity Model

## Purpose

Entities represent all important objects discovered during an investigation.

Everything in ODIN revolves around entities.

---

# Entity Types

## User

Fields

- SID
- Username
- Domain
- Privileges

---

## Host

Fields

- Hostname
- IP
- OS
- Asset ID

---

## Process

Fields

- PID
- Parent PID
- Executable
- Command Line
- Hash

---

## File

Fields

- Path
- SHA256
- Size
- Signature

---

## Registry

Fields

- Key
- Value
- Hive

---

## Service

Fields

- Name
- Status
- Binary Path

---

## Scheduled Task

Fields

- Task Name
- Trigger
- Command

---

## Network

Fields

- Source IP
- Destination IP
- Port
- Protocol

---

## Domain

Fields

- Domain
- Registrar
- Reputation

---

## Email

Fields

- Sender
- Recipient
- Subject
- Attachment

---

# Shared Fields

Every entity contains

- UUID
- Entity Type
- Confidence
- First Seen
- Last Seen
- Source
