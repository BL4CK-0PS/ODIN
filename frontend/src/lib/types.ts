export type Severity = "Critical" | "High" | "Medium" | "Low" | "Informational";
export type IncidentStatus = "New" | "Investigating" | "Contained" | "Eradicated" | "Recovered" | "Closed";
export type EvidenceType = "Log" | "NetworkCapture" | "FileSystemArtifact" | "MemoryDump" | "ThreatIntelReport" | "UserReport";

export interface CanonicalIncident {
  id: string;
  title: string;
  description: string;
  severity: Severity;
  status: IncidentStatus;
  created_at: string;
  tags: string[];
  evidence_ids: string[];
  entity_ids: string[];
  mitre_techniques: string[];
}

export interface Evidence {
  id: string;
  incident_id: string;
  source: string;
  content: string;
  content_type: EvidenceType;
  trust_score: number;
  collected_at: string;
}

export interface Entity {
  id: string;
  name: string;
  entity_type: string;
  metadata: Record<string, unknown>;
  created_at: string;
}

export interface MemoryObject {
  id: string;
  incident_id: string;
  summary: string;
  confidence: number;
  version: number;
  created_at: string;
}

export interface RankedResult {
  memory: MemoryObject;
  score: { overall: number; structural: number; semantic: number; context: number };
  reasons: string[];
}

export type Page =
  | "dashboard"
  | "investigations"
  | "investigation-detail"
  | "threat-memory"
  | "knowledge-explorer"
  | "search"
  | "settings";

export interface RouterState {
  page: Page;
  params: Record<string, string>;
}
