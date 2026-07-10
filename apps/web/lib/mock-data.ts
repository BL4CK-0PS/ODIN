import type { CanonicalIncident, MemoryObject, RankedResult, Severity, IncidentStatus } from "./types";
import type { GraphNode, GraphEdge } from "@/stores/graph";

import mock_incident_1 from "../../../mock_datas/mock_incident_1.json";
import mock_incident_2 from "../../../mock_datas/mock_incident_2.json";
import mock_ransomware from "../../../mock_datas/mock_ransomware.json";
import mock_phishing from "../../../mock_datas/mock_phishing_campaign.json";
import mock_data_exfil from "../../../mock_datas/mock_data_exfil.json";
import mock_log4j from "../../../mock_datas/mock_log4j_exploit.json";
import mock_container from "../../../mock_datas/mock_container_escape.json";
import mock_lateral from "../../../mock_datas/mock_lateral_movement.json";
import mock_insider from "../../../mock_datas/mock_insider_threat.json";
import mock_supplychain from "../../../mock_datas/mock_supplychain_dll.json";
import mock_iot from "../../../mock_datas/mock_iot_botnet.json";

type MockFile = {
  title: string;
  description: string;
  severity: string;
  evidence: { source: string; content: string; content_type: string }[];
};

const STATUSES: IncidentStatus[] = [
  "Investigating",
  "Contained",
  "New",
  "Investigating",
  "Closed",
];

function extractTechniques(desc: string): string[] {
  const matches = desc.match(/T\d{4}(?:\.\d{3})?/g);
  return matches ? Array.from(new Set(matches)) : [];
}

function deriveTags(title: string, desc: string): string[] {
  const tags = new Set<string>();
  const lower = (title + " " + desc).toLowerCase();

  if (lower.includes("phishing") || lower.includes("spearphish")) tags.add("phishing");
  if (lower.includes("ransomware")) tags.add("ransomware");
  if (lower.includes("lateral movement") || lower.includes("kerberoast")) tags.add("lateral-movement");
  if (lower.includes("exfiltrat") || lower.includes("data exfil")) tags.add("data-exfiltration");
  if (lower.includes("dns tunnel")) tags.add("dns-tunneling");
  if (lower.includes("insider")) tags.add("insider-threat");
  if (lower.includes("supply chain")) tags.add("supply-chain");
  if (lower.includes("iot") || lower.includes("botnet")) tags.add("iot");
  if (lower.includes("container") || lower.includes("kubernetes")) tags.add("container");
  if (lower.includes("cloud") || lower.includes("aws")) tags.add("cloud");
  if (lower.includes("credential") || lower.includes("mfa")) tags.add("credential-theft");
  if (lower.includes("c2") || lower.includes("command and control")) tags.add("c2");
  if (lower.includes("powershell")) tags.add("powershell");

  if (tags.size === 0) tags.add("general");
  return Array.from(tags);
}

function mapSeverity(s: string): Severity {
  const normalized = s.charAt(0).toUpperCase() + s.slice(1).toLowerCase();
  if (["Critical", "High", "Medium", "Low", "Informational"].includes(normalized)) {
    return normalized as Severity;
  }
  return "Medium";
}

function mapContentType(t: string): "Log" | "NetworkCapture" | "FileSystemArtifact" {
  const lower = t.toLowerCase();
  if (lower.includes("network")) return "NetworkCapture";
  if (lower.includes("file")) return "FileSystemArtifact";
  return "Log";
}

const rawFiles: MockFile[] = [
  mock_incident_1,
  mock_incident_2,
  mock_ransomware,
  mock_phishing,
  mock_data_exfil,
  mock_log4j,
  mock_container,
  mock_lateral,
  mock_insider,
  mock_supplychain,
  mock_iot,
];

export const mockIncidents: CanonicalIncident[] = rawFiles.map((file, i) => ({
  id: `inc-${String(i + 1).padStart(3, "0")}`,
  title: file.title,
  description: file.description,
  severity: mapSeverity(file.severity),
  status: STATUSES[i % STATUSES.length],
  created_at: new Date(2024, 10, 1 + i * 2, 8 + i, 30).toISOString(),
  updated_at: new Date(2024, 11, 1 + i, 14 + i, 0).toISOString(),
  tags: deriveTags(file.title, file.description),
  evidence_ids: file.evidence.map((_, j) => `ev-${String(i + 1).padStart(3, "0")}-${j}`),
  entity_ids: Array.from(new Set(file.evidence.flatMap((_, j) => [`ent-${String(i + 1).padStart(3, "0")}-${j}`]))).slice(0, Math.min(6, file.evidence.length)),
  mitre_techniques: extractTechniques(file.description),
}));

export function getMockIncident(id: string): CanonicalIncident {
  return mockIncidents.find((inc) => inc.id === id) ?? mockIncidents[0];
}

export function getMockEvidence(incidentId: string) {
  const idx = mockIncidents.findIndex((inc) => inc.id === incidentId);
  const file = rawFiles[idx >= 0 ? idx : 0];
  return file.evidence.map((ev, j) => ({
    id: `ev-${String((idx >= 0 ? idx : 0) + 1).padStart(3, "0")}-${j}`,
    incident_id: incidentId,
    source: ev.source,
    content: ev.content,
    content_type: mapContentType(ev.content_type),
    trust_score: 0.65 + Math.random() * 0.3,
    collected_at: new Date(2024, 11, 1, 8 + j * 5, 30 + j * 7).toISOString(),
  }));
}

export const mockTimeline = (() => {
  const evidence = getMockEvidence("inc-001");
  return {
    incident_id: "inc-001",
    events: evidence.map((ev) => ({
      id: ev.id,
      source: ev.source,
      type: ev.content_type,
      collected_at: ev.collected_at,
      trust_score: ev.trust_score,
      content: ev.content,
    })),
  };
})();

export const mockMemory: MemoryObject = {
  id: "mem-001",
  incident_id: "inc-001",
  summary:
    "Suspicious encoded PowerShell command executed in HR department computer. " +
    "Evidence includes Sysmon process creation events showing encoded command download, " +
    "and network capture showing connection to C2 server. Indicators: 192.168.1.55, c2-server.com, T1059.",
  context: { title: mock_incident_1.title, severity: mock_incident_1.severity },
  confidence: 0.88,
  version: 2,
  created_at: "2024-12-03T14:20:00Z",
};

const graphNodes: GraphNode[] = [
  { id: "g-n1", type: "incident", label: mock_incident_1.title.slice(0, 30) },
  { id: "g-n2", type: "ipaddress", label: "192.168.1.55" },
  { id: "g-n3", type: "domain", label: "c2-server.com" },
  { id: "g-n4", type: "process", label: "powershell.exe" },
  { id: "g-n5", type: "file", label: "malware.exe" },
  { id: "g-n6", type: "evidence", label: "Sysmon Event 1" },
  { id: "g-n7", type: "evidence", label: "Network Capture" },
  { id: "g-n8", type: "hostname", label: "HR-WS-042" },
  { id: "g-n9", type: "user", label: "admin" },
  { id: "g-n10", type: "domain", label: "c2-server.com:80" },
];

const graphEdges: GraphEdge[] = [
  { source: "g-n1", target: "g-n6", type: "GeneratedBy", label: "generated_by" },
  { source: "g-n1", target: "g-n7", type: "GeneratedBy", label: "generated_by" },
  { source: "g-n6", target: "g-n4", type: "Observed", label: "observed" },
  { source: "g-n6", target: "g-n8", type: "Targets", label: "targets" },
  { source: "g-n7", target: "g-n3", type: "CommunicatesWith", label: "c2_server" },
  { source: "g-n7", target: "g-n5", type: "Dropped", label: "dropped" },
  { source: "g-n4", target: "g-n9", type: "ExecutedBy", label: "executed_by" },
  { source: "g-n3", target: "g-n10", type: "ResolvesTo", label: "resolves_to" },
];

export const mockGraph = { nodes: graphNodes, edges: graphEdges };

export const mockPlaybooks = {
  incident_id: "inc-001",
  playbooks: [
    {
      name: "Incident Response - Phishing / Remote Execution",
      steps: [
        "Isolate affected workstation from network",
        "Block C2 domain and IP at perimeter firewall",
        "Collect Sysmon and network logs for forensic preservation",
        "Scan for lateral movement indicators across the subnet",
        "Reset credentials for compromised user accounts",
        "Run full endpoint scan on affected host",
        "Update threat intelligence IOC feeds",
        "Document findings and escalate if data breach confirmed",
      ],
    },
  ],
};

export const mockSimilarResults: RankedResult[] = [
  {
    memory: {
      id: "mem-010",
      incident_id: "inc-010",
      summary:
        "Cobalt Strike beacon lateral movement via PsExec across finance network. " +
        "Initial access through phishing email with macro-enabled document. " +
        "C2 communication over HTTPS to known APT29 infrastructure.",
      context: { title: "APT29 Lateral Movement - Finance" },
      confidence: 0.89,
      version: 3,
      created_at: "2024-09-15T10:00:00Z",
    },
    score: {
      overall: 0.87,
      structural: 0.91,
      semantic: 0.84,
      context: 0.86,
    },
    reasons: [
      "Similar C2 infrastructure (port 80/443)",
      "Same initial access vector (encoded PowerShell)",
      "Comparable network indicators",
    ],
  },
  {
    memory: {
      id: "mem-011",
      incident_id: "inc-011",
      summary:
        "Encoded PowerShell download cradle used to deploy second-stage payload. " +
        "Connection from internal host to external server on port 80. " +
        "Malware binary dropped to local temp directory.",
      context: { title: "PowerShell Download Cradle - DevOps" },
      confidence: 0.82,
      version: 2,
      created_at: "2024-06-20T14:00:00Z",
    },
    score: {
      overall: 0.78,
      structural: 0.82,
      semantic: 0.75,
      context: 0.77,
    },
    reasons: [
      "Identical encoded command pattern",
      "Same C2 domain family",
      "Matching TTPs (T1059, T1059.001)",
    ],
  },
  {
    memory: {
      id: "mem-012",
      incident_id: "inc-012",
      summary:
        "Suspicious network connection from workstation to unknown external IP on port 80. " +
        "Subsequent malware download detected by EDR. DLL injection into legitimate process.",
      context: { title: "Network-Based Malware Delivery - Sales" },
      confidence: 0.71,
      version: 1,
      created_at: "2024-08-05T09:00:00Z",
    },
    score: {
      overall: 0.64,
      structural: 0.60,
      semantic: 0.68,
      context: 0.63,
    },
    reasons: [
      "Similar network connection pattern",
      "DLL injection technique match",
      "Same severity classification",
    ],
  },
];

export const mockThreatMemories: MemoryObject[] = mockIncidents.map((inc) => {
  const file = rawFiles.find((f) => f.title === inc.title);
  const techs = inc.mitre_techniques.slice(0, 3).join(", ");
  return {
    id: `mem-${inc.id.replace("inc-", "")}`,
    incident_id: inc.id,
    summary:
      (file ? file.description.slice(0, 200) : inc.description.slice(0, 200)) +
      (techs ? ` [${techs}]` : ""),
    context: { title: inc.title, severity: inc.severity },
    confidence: +(0.7 + Math.random() * 0.25).toFixed(2),
    version: 1 + Math.floor(Math.random() * 3),
    created_at: inc.created_at,
  };
});

export const mockStats = {
  investigations: mockIncidents.length,
  memories: mockThreatMemories.length,
  entities: mockIncidents.reduce((sum, inc) => sum + inc.entity_ids.length, 0),
  matches: mockSimilarResults.length,
};
