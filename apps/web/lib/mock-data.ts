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

export function getMockTimeline(incidentId: string) {
  const evidence = getMockEvidence(incidentId);
  return {
    incident_id: incidentId,
    events: evidence.map((ev) => ({
      id: ev.id,
      source: ev.source,
      type: ev.content_type,
      collected_at: ev.collected_at,
      trust_score: ev.trust_score,
      content: ev.content,
    })),
  };
}

export function getMockMemory(incidentId: string): MemoryObject {
  const inc = getMockIncident(incidentId);
  const techs = inc.mitre_techniques.slice(0, 3).join(", ");
  return {
    id: `mem-${incidentId.replace("inc-", "")}`,
    incident_id: incidentId,
    summary: inc.description.slice(0, 250) + (techs ? ` [${techs}]` : ""),
    context: { title: inc.title, severity: inc.severity },
    confidence: +(0.7 + Math.random() * 0.25).toFixed(2),
    version: 1 + Math.floor(Math.random() * 3),
    created_at: inc.created_at,
  };
}

function extractIocsFromEvidence(evidence: { source: string; content: string; content_type: string }[]) {
  const ips = new Set<string>();
  const domains = new Set<string>();
  const processes = new Set<string>();
  const files = new Set<string>();
  const users = new Set<string>();

  for (const ev of evidence) {
    const text = ev.content;
    const ipMatches = text.match(/\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b/g);
    if (ipMatches) for (const ip of ipMatches) ips.add(ip);

    const domainMatches = text.match(/\b([a-zA-Z0-9][-a-zA-Z0-9]*\.)+[a-zA-Z]{2,}\b/g);
    if (domainMatches) for (const d of domainMatches) if (!d.match(/^\d/)) domains.add(d);

    const procMatches = text.match(/\b[\w.-]+\.exe\b/gi);
    if (procMatches) for (const p of procMatches) processes.add(p.toLowerCase());

    const fileMatches = text.match(/\b[\w.-]+\.(dll|exe|lnk|docm|ps1|bat|vbs|js)\b/gi);
    if (fileMatches) for (const f of fileMatches) files.add(f.toLowerCase());

    const userMatch = text.match(/\buser=(\w+)/i);
    if (userMatch) users.add(userMatch[1]);
  }

  return { ips, domains, processes, files, users };
}

function buildGraphFromIncident(incidentId: string): { nodes: GraphNode[]; edges: GraphEdge[] } {
  const inc = getMockIncident(incidentId);
  const file = rawFiles.find((f) => f.title === inc.title) ?? rawFiles[0];
  const iocs = extractIocsFromEvidence(file.evidence);

  const nodes: GraphNode[] = [];
  const edges: GraphEdge[] = [];
  let nid = 0;

  const nidFor = () => `g-n${++nid}`;

  const incidentNodeId = nidFor();
  nodes.push({ id: incidentNodeId, type: "incident", label: inc.title.slice(0, 30) });

  const evidenceNodeIds: string[] = [];
  for (const ev of file.evidence) {
    const evId = nidFor();
    evidenceNodeIds.push(evId);
    nodes.push({ id: evId, type: "evidence", label: ev.source.slice(0, 25) });
    edges.push({ source: incidentNodeId, target: evId, type: "GeneratedBy", label: "generated_by" });
  }

  for (const ip of Array.from(iocs.ips).slice(0, 4)) {
    const ipId = nidFor();
    nodes.push({ id: ipId, type: "ipaddress", label: ip });
    for (const evId of evidenceNodeIds.slice(0, 2)) {
      edges.push({ source: evId, target: ipId, type: "Observed", label: "observed" });
    }
  }

  for (const domain of Array.from(iocs.domains).slice(0, 3)) {
    const domId = nidFor();
    nodes.push({ id: domId, type: "domain", label: domain });
    for (const evId of evidenceNodeIds.slice(0, 2)) {
      edges.push({ source: evId, target: domId, type: "CommunicatesWith", label: "c2_server" });
    }
  }

  for (const proc of Array.from(iocs.processes).slice(0, 3)) {
    const procId = nidFor();
    nodes.push({ id: procId, type: "process", label: proc });
    for (const evId of evidenceNodeIds.slice(0, 2)) {
      edges.push({ source: evId, target: procId, type: "Observed", label: "observed" });
    }
  }

  for (const file of Array.from(iocs.files).slice(0, 3)) {
    const fileId = nidFor();
    nodes.push({ id: fileId, type: "file", label: file });
    for (const evId of evidenceNodeIds.slice(0, 2)) {
      edges.push({ source: evId, target: fileId, type: "Dropped", label: "dropped" });
    }
  }

  for (const user of Array.from(iocs.users).slice(0, 2)) {
    const userId = nidFor();
    nodes.push({ id: userId, type: "user", label: user });
  }

  return { nodes, edges };
}

export function getMockGraph(incidentId: string) {
  return buildGraphFromIncident(incidentId);
}

export function getMockPlaybooks(incidentId: string) {
  const inc = getMockIncident(incidentId);
  const file = rawFiles.find((f) => f.title === inc.title) ?? rawFiles[0];
  const evidenceSources = file.evidence.map((e) => e.source);
  const title = inc.title;
  const name = title.length > 40 ? `Incident Response — ${title.slice(0, 36)}...` : `Incident Response — ${title}`;

  const steps: string[] = [];

  const hasNetwork = evidenceSources.some((s) => s.toLowerCase().includes("network"));
  const hasSysmon = evidenceSources.some((s) => s.toLowerCase().includes("sysmon"));
  const hasEmail = evidenceSources.some((s) => s.toLowerCase().includes("email"));
  const hasEdr = evidenceSources.some((s) => s.toLowerCase().includes("edr"));
  const hasFile = evidenceSources.some((s) => s.toLowerCase().includes("file"));

  if (hasEmail) steps.push("Analyze email gateway logs for phishing indicators and malicious attachments");
  if (hasSysmon) steps.push("Review Sysmon process creation and network events for execution chain");
  if (hasEdr) steps.push("Investigate EDR alerts for suspicious process behavior and file writes");
  if (hasNetwork) steps.push("Capture and analyze network traffic for C2 communication patterns");
  if (hasFile) steps.push("Examine file system artifacts and audit logs for unauthorized changes");
  steps.push("Isolate affected endpoints from the network");
  steps.push("Block identified IOCs at perimeter firewall and DNS");
  steps.push("Collect forensic evidence for preservation");
  if (hasSysmon || hasEdr) steps.push("Scan environment for lateral movement indicators");
  steps.push("Reset credentials for compromised accounts");
  steps.push("Document findings and escalate to incident commander");

  return {
    incident_id: incidentId,
    playbooks: [
      {
        name,
        steps,
      },
    ],
  };
}

export function getMockSimilarResults(incidentId: string) {
  const inc = getMockIncident(incidentId);
  const file = rawFiles.find((f) => f.title === inc.title) ?? rawFiles[0];
  const results: RankedResult[] = [];

  const otherFiles = rawFiles.filter((_, i) => rawFiles[i].title !== file.title);
  const picked = otherFiles.slice(0, 3);

  for (let k = 0; k < picked.length; k++) {
    const other = picked[k];
    const otherInc = mockIncidents.find((m) => m.title === other.title);
    const techs = (otherInc?.mitre_techniques ?? []).slice(0, 3).join(", ");
    const iocs = extractIocsFromEvidence(other.evidence);
    const reasons: string[] = [];

    const ourIocs = extractIocsFromEvidence(file.evidence);
    const sharedIps = Array.from(ourIocs.ips).filter((ip) => iocs.ips.has(ip));
    const sharedDomains = Array.from(ourIocs.domains).filter((d) => iocs.domains.has(d));
    const sharedTechs = (inc.mitre_techniques ?? []).filter((t) => (otherInc?.mitre_techniques ?? []).includes(t));

    if (sharedIps.length) reasons.push(`Shared IP indicators: ${sharedIps.slice(0, 2).join(", ")}`);
    if (sharedDomains.length) reasons.push(`Shared domain indicators: ${sharedDomains.slice(0, 2).join(", ")}`);
    if (sharedTechs.length) reasons.push(`Matching TTPs: ${sharedTechs.slice(0, 2).join(", ")}`);
    if (reasons.length === 0) reasons.push(`Similar severity (${otherInc?.severity ?? "N/A"}) and attack pattern`);

    results.push({
      memory: {
        id: `mem-${(otherInc?.id ?? `0${k + 1}`).replace("inc-", "")}`,
        incident_id: otherInc?.id ?? `inc-0${k + 1}`,
        summary: other.description.slice(0, 200) + (techs ? ` [${techs}]` : ""),
        context: { title: other.title, severity: otherInc?.severity },
        confidence: +(0.7 + Math.random() * 0.25).toFixed(2),
        version: 1 + Math.floor(Math.random() * 3),
        created_at: otherInc?.created_at ?? "2024-12-01T00:00:00Z",
      },
      score: {
        overall: +(0.6 + Math.random() * 0.3).toFixed(2),
        structural: +(0.6 + Math.random() * 0.3).toFixed(2),
        semantic: +(0.6 + Math.random() * 0.3).toFixed(2),
        context: +(0.6 + Math.random() * 0.3).toFixed(2),
      },
      reasons,
    });
  }

  return results;
}

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
  matches: 3,
};
