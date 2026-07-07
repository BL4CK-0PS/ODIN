# Canonical Incident

A canonical incident represents the **complete idealized structure** of an ODIN incident after full enrichment and analysis.

```yaml
incident:
  id: "inc_01h2x3..."
  title: "Spear-phishing campaign leading to Cobalt Strike deployment"
  severity: "Critical"
  status: "Closed"
  source: "Microsoft Sentinel"
  source_id: "alert_abc123"
  created_at: "2026-06-15T08:23:00Z"
  closed_at: "2026-06-17T14:30:00Z"

analyst:
  assigned: "user_jordan"
  notes: 12
  total_time_hours: 8.5

techniques:
  - T1566.001: "Spearphishing Attachment"
  - T1204.002: "User Execution: Malicious File"
  - T1059.001: "PowerShell"
  - T1071.001: "Web Protocols"
  - T1055.001: "Process Injection"
  - T1041: "Exfiltration Over C2 Channel"

observables:
  - type: "sha256", value: "a1b2c3...", enrichment: { vt_malicious: 12 }
  - type: "ip", value: "5.6.7.8", enrichment: { vt_malicious: 8, asn: "AS1234" }
  - type: "domain", value: "evil.example.com", enrichment: { vt_malicious: 15 }
  - type: "email", value: "phish@evil.example.com"

evidence:
  - type: "file", name: "invoice.pdf.lnk", file_hash: "..."
  - type: "log", source: "EDR", entries: 45
  - type: "artifact", name: "powershell_script.txt"
  - type: "screenshot", name: "c2_beacon_config"
  - type: "note", content: "C2 beacon checked in every 60s"

timeline:
  - 08:23: Alert created (Sentinel)
  - 08:25: Phishing email detected (email gateway)
  - 08:28: User clicked link (EDR event)
  - 08:29: PowerShell execution (EDR event)
  - 08:31: C2 beacon outbound (firewall log)
  - 08:35: Process injection (EDR event)
  - 08:45: Incident triaged by Alex
  - 09:15: Host isolated
  - 10:30: Cobalt Strike profile identified
  - 14:00: Beacon removed, host reimaged
  - Day+1: IOC sweep across enterprise
  - Day+2: Incident closed

similar_incidents:
  - id: "inc_09f8a...", score: 0.92, title: "Cobalt Strike via phishing"
  - id: "inc_02b7c...", score: 0.87, title: "Qakbot with Cobalt Strike"
  - id: "inc_04d1e...", score: 0.81, title: "Excel macro -> PowerShell"

narrative:
  executive: |
    On June 15, 2026, an employee received a spear-phishing email with a malicious
    attachment. Opening the attachment triggered a PowerShell script that downloaded
    and executed Cobalt Strike Beacon. The attacker maintained persistence for
    approximately 6 hours before the host was isolated...
  technical: |
    Initial access via T1566.001 (spearphishing attachment). LNK file executed
    mshta.exe to fetch PowerShell payload from evil.example.com...
  remediation: |
    1. Isolate affected host
    2. Remove Cobalt Strike artifacts
    3. Reset affected user credentials
    4. Block C2 domains at proxy
    5. Deploy additional phishing detection rules
```
