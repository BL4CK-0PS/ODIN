const BASE = "/api/v1";

async function request<T>(path: string, opts?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${path}`, {
    headers: { "Content-Type": "application/json", ...opts?.headers },
    ...opts,
  });
  const json = await res.json();
  if (!json.success) throw new Error(json.error || "Unknown error");
  return json.data as T;
}

export const api = {
  health: () => request<{ status: string }>("/system/health"),
  uploadIncident: (body: { title: string; description: string; severity: string; evidence: { source: string; content: string; content_type: string }[] }) =>
    request<{ id: string }>("/incidents/upload", { method: "POST", body: JSON.stringify(body) }),
  getIncident: (id: string) => request<Record<string, unknown>>(`/incidents/${id}`),
  getTimeline: (id: string) => request<{ incident_id: string; events: unknown[] }>(`/incidents/${id}/timeline`),
  getMemory: (id: string) => request<unknown>(`/incidents/${id}/memory`),
  getGraph: (id: string) => request<{ nodes: unknown[]; edges: unknown[] }>(`/incidents/${id}/graph`),
  getPlaybooks: (id: string) => request<unknown>(`/incidents/${id}/playbooks`),
  searchSimilar: (incident_id: string, top_k = 5) =>
    request<{ results: unknown[] }>("/incidents/search", { method: "POST", body: JSON.stringify({ incident_id, top_k }) }),
};
