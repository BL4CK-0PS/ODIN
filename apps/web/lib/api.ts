import type { ApiResponse, CanonicalIncident, RankedResult, MemoryObject } from "./types";
import type { GraphData } from "@/stores/graph";
import { mockIncidents, getMockIncident, getMockMemory, getMockGraph, getMockSimilarResults } from "./mock-data";

const BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000/api/v1";
const USE_MOCKS = process.env.NEXT_PUBLIC_USE_MOCKS === "true";
const DEFAULT_TIMEOUT = 30_000;
const MAX_RETRIES = 3;
const BASE_DELAY = 500;

export class ApiError extends Error {
  constructor(
    message: string,
    public status?: number,
    public code?: string,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

class NetworkError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "NetworkError";
  }
}

async function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function request<T>(
  path: string,
  opts?: RequestInit & { timeout?: number },
): Promise<T> {
  const timeout = opts?.timeout ?? DEFAULT_TIMEOUT;
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeout);

  let lastError: Error | null = null;

  for (let attempt = 0; attempt < MAX_RETRIES; attempt++) {
    try {
      if (attempt > 0) {
        const jitter = Math.random() * 200;
        await delay(Math.min(BASE_DELAY * Math.pow(2, attempt) + jitter, 10_000));
      }

      const res = await fetch(`${BASE}${path}`, {
        headers: { "Content-Type": "application/json", ...opts?.headers },
        signal: controller.signal,
        ...opts,
      });

      if (!res.ok && res.status >= 500) {
        throw new ApiError(
          `Server error: ${res.status}`,
          res.status,
          "SERVER_ERROR",
        );
      }

      const json: ApiResponse<T> = await res.json();

      if (!json.success) {
        throw new ApiError(
          json.error || "Unknown error",
          res.status,
          "API_ERROR",
        );
      }

      return json.data as T;
    } catch (err) {
      lastError = err as Error;

      if (err instanceof ApiError && err.status && err.status < 500) {
        throw err; // Don't retry client errors (4xx)
      }

      if (err instanceof DOMException && err.name === "AbortError") {
        throw new NetworkError(`Request timed out after ${timeout}ms`);
      }

      if (attempt === MAX_RETRIES - 1) {
        throw err; // Last attempt, propagate
      }
    } finally {
      if (attempt === MAX_RETRIES - 1) {
        clearTimeout(timeoutId);
      }
    }
  }

  clearTimeout(timeoutId);
  throw lastError || new Error("Unknown error");
}

export interface TimelineData {
  incident_id: string;
  events: unknown[];
}

export interface PlaybookResponse {
  incident_id: string;
  playbooks: unknown[];
}

export const api = {
  health: () => request<{ status: string; version: string }>("/system/health"),

  uploadIncident: (body: {
    title: string;
    description: string;
    severity: string;
    evidence: { source: string; content: string; content_type: string }[];
  }) =>
    request<{
      id: string;
      title: string;
      severity: string;
      status: string;
      evidence_count: number;
      entity_count: number;
    }>("/incidents/upload", {
      method: "POST",
      body: JSON.stringify(body),
      timeout: 60_000,
    }),

  getIncident: (id: string) =>
    request<CanonicalIncident>(`/incidents/${id}`),

  getTimeline: (id: string) =>
    request<TimelineData>(`/incidents/${id}/timeline`),

  getMemory: (id: string) =>
    request<MemoryObject>(`/incidents/${id}/memory`),

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

  updateStatus: (id: string, status: string) =>
    request<{ incident_id: string; old_status: string; new_status: string }>(
      `/incidents/${id}/status`,
      {
        method: "POST",
        body: JSON.stringify({ status }),
      },
    ),

  getConsolidationStats: () =>
    request<{
      total_memories: number;
      expired_purged: number;
      versions_pruned: number;
      memories_consolidated: number;
      consolidation_runs: number;
      last_consolidation: string | null;
      ttl_config: Record<string, string>;
    }>("/consolidation/stats"),

  getReportHtml: async (id: string): Promise<string> => {
    const timeout = DEFAULT_TIMEOUT;
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), timeout);

    try {
      const res = await fetch(`${BASE}/incidents/${id}/report`, {
        headers: { "Accept": "text/html" },
        signal: controller.signal,
      });

      if (!res.ok) {
        throw new ApiError(`Report generation failed: ${res.status}`, res.status);
      }

      return await res.text();
    } finally {
      clearTimeout(timeoutId);
    }
  },
};

export const mockApi = {
  health: () => Promise.resolve({ status: "ok", version: "0.1.0" }),

  uploadIncident: (body: {
    title: string;
    description: string;
    severity: string;
    evidence: { source: string; content: string; content_type: string }[];
  }) =>
    Promise.resolve({
      id: `inc-${Date.now()}`,
      title: body.title,
      severity: body.severity,
      status: "New",
      evidence_count: body.evidence.length,
      entity_count: 0,
    }),

  getIncident: (id: string) => {
    const inc = getMockIncident(id);
    return inc ? Promise.resolve(inc) : Promise.reject(new ApiError("Not found", 404));
  },

  getTimeline: (id: string) =>
    Promise.resolve({ incident_id: id, events: [] }),

  getMemory: (id: string) =>
    Promise.resolve(getMockMemory(id)),

  getGraph: (id: string) =>
    Promise.resolve(getMockGraph(id)),

  getGlobalGraph: () =>
    Promise.resolve(getMockGraph(mockIncidents[0]?.id || "inc-001")),

  getPlaybooks: (id: string) =>
    Promise.resolve({ incident_id: id, playbooks: [] }),

  searchSimilar: (incident_id: string, top_k = 5) =>
    Promise.resolve({
      results: getMockSimilarResults(incident_id).slice(0, top_k),
    }),

  postFeedback: (_id: string, _feedback: string, _rating: number) =>
    Promise.resolve("Feedback recorded"),

  updateStatus: (id: string, status: string) =>
    Promise.resolve({ incident_id: id, old_status: "New", new_status: status }),

  getConsolidationStats: () =>
    Promise.resolve({
      total_memories: 11,
      expired_purged: 0,
      versions_pruned: 0,
      memories_consolidated: 0,
      consolidation_runs: 5,
      last_consolidation: new Date(Date.now() - 3600_000).toISOString(),
      ttl_config: { critical: "365 days", high: "180 days", medium: "90 days", low: "30 days" },
    }),

  getReportHtml: (_id: string) =>
    Promise.resolve("<html><body><h1>Mock Report</h1></body></html>"),
};

export const clientApi = USE_MOCKS ? mockApi : api;
