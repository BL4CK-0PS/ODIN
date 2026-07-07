import type { ApiResponse, CanonicalIncident, RankedResult } from "./types";

const BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000/api/v1";

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
  health: () => request<{ status: string }>("/system/health"),

  version: () => request<{ version: string }>("/system/version"),

  uploadIncident: (body: {
    title: string;
    description: string;
    severity: string;
    evidence: { source: string; content: string; content_type: string }[];
  }) =>
    request<{ id: string }>("/incidents/upload", {
      method: "POST",
      body: JSON.stringify(body),
    }),

  getIncident: (id: string) => request<CanonicalIncident>(`/incidents/${id}`),

  getTimeline: (id: string) =>
    request<{ incident_id: string; events: unknown[] }>(`/incidents/${id}/timeline`),

  getMemory: (id: string) => request<unknown>(`/incidents/${id}/memory`),

  getGraph: (id: string) =>
    request<{ nodes: unknown[]; edges: unknown[] }>(`/incidents/${id}/graph`),

  getPlaybooks: (id: string) =>
    request<{ incident_id: string; playbooks: unknown[] }>(`/incidents/${id}/playbooks`),

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
