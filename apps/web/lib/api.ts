import type { ApiResponse, CanonicalIncident, RankedResult, MemoryObject } from "./types";
import type { GraphData } from "@/stores/graph";

const BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000/api/v1";

export interface TimelineData {
  incident_id: string;
  events: unknown[];
}

export interface PlaybookResponse {
  incident_id: string;
  playbooks: unknown[];
}

async function request<T>(path: string, opts?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${path}`, {
    headers: { "Content-Type": "application/json", ...opts?.headers },
    ...opts,
  });
  const json: ApiResponse<T> = await res.json();
  if (!json.success) throw new Error(json.error || "Unknown error");
  return json.data as T;
}

export const api = {
  health: () => request<{ status: string; version: string }>("/system/health"),

  version: () => request<{ version: string; name: string; build: string }>("/system/version"),

  uploadIncident: (body: {
    title: string;
    description: string;
    severity: string;
    evidence: { source: string; content: string; content_type: string }[];
  }) =>
    request<{ id: string; title: string; severity: string; status: string; evidence_count: number; entity_count: number }>("/incidents/upload", {
      method: "POST",
      body: JSON.stringify(body),
    }),

  getIncident: (id: string) => request<CanonicalIncident>(`/incidents/${id}`),

  getTimeline: (id: string) =>
    request<TimelineData>(`/incidents/${id}/timeline`),

  getMemory: (id: string) => request<MemoryObject>(`/incidents/${id}/memory`),

  getGraph: (id: string) =>
    request<GraphData>(`/incidents/${id}/graph`),

  getGlobalGraph: () =>
    request<GraphData>("/graph"),

  getPlaybooks: (id: string) =>
    request<PlaybookResponse>(`/incidents/${id}/playbooks`),

  searchSimilar: (incident_id: string, top_k = 5) =>
    request<{ results: RankedResult[] }>("/incidents/search", {
      method: "POST",
      body: JSON.stringify({ incident_id, top_k }),
    }),

  postFeedback: (id: string, feedback: string, rating: number) =>
    request<string>(`/incidents/${id}/feedback`, {
      method: "POST",
      body: JSON.stringify({ feedback, rating }),
    }),
};
