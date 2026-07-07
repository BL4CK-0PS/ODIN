export type Severity = "Critical" | "High" | "Medium" | "Low" | "Informational";
export type IncidentStatus = "New" | "Investigating" | "Contained" | "Eradicated" | "Recovered" | "Closed";
export type EvidenceType = "Log" | "NetworkCapture" | "FileSystemArtifact" | "MemoryDump" | "ThreatIntelReport" | "UserReport";
export type RelationshipType = "RelatedTo" | "DerivedFrom" | "References" | "PartOf" | "Mitigates" | "Exploits" | "Indicates";
export type EntityType = "IpAddress" | "Domain" | "Hash" | "Hostname" | "User" | "Process" | "File" | "NetworkConnection" | "Artifact";

export interface CanonicalIncident {
  id: string;
  title: string;
  description: string;
  severity: Severity;
  status: IncidentStatus;
  created_at: string;
  updated_at: string;
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
  entity_type: EntityType;
  metadata: Record<string, unknown>;
  created_at: string;
}

export interface MemoryObject {
  id: string;
  incident_id: string;
  summary: string;
  context: Record<string, unknown>;
  confidence: number;
  version: number;
  created_at: string;
}

export interface HybridScore {
  overall: number;
  structural: number;
  semantic: number;
  context: number;
}

export interface RankedResult {
  memory: MemoryObject;
  score: HybridScore;
  reasons: string[];
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}
