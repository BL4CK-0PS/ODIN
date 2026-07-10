import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api";
import { useInvestigationStore } from "@/stores/investigation";
import { mockIncidents, getMockIncident, mockTimeline, mockMemory, mockGraph, mockPlaybooks } from "@/lib/mock-data";
import type { CanonicalIncident } from "@/lib/types";

export function useInvestigation(id: string) {
  const setIncident = useInvestigationStore((s) => s.setIncident);

  return useQuery({
    queryKey: ["investigation", id],
    queryFn: async () => {
      try {
        const [incident, timeline, memory, graph, playbooks] = await Promise.all([
          api.getIncident(id),
          api.getTimeline(id),
          api.getMemory(id),
          api.getGraph(id),
          api.getPlaybooks(id),
        ]);
        setIncident(incident as unknown as CanonicalIncident);
        return { incident, timeline, memory, graph, playbooks };
      } catch {
        const incident = getMockIncident(id);
        setIncident(incident);
        return {
          incident,
          timeline: mockTimeline,
          memory: mockMemory,
          graph: mockGraph,
          playbooks: mockPlaybooks,
        };
      }
    },
    enabled: !!id,
    retry: 1,
    staleTime: 15_000,
  });
}

export function useInvestigations() {
  return useQuery({
    queryKey: ["investigations"],
    queryFn: async () => {
      try {
        const res = await fetch("/api/v1/incidents");
        const json = await res.json();
        if (!json.success) throw new Error(json.error || "Failed to fetch investigations");
        return json.data as CanonicalIncident[];
      } catch {
        return mockIncidents;
      }
    },
    retry: 1,
    staleTime: 15_000,
  });
}

export function useUploadInvestigation() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: {
      title: string;
      description: string;
      severity: string;
      evidence: { source: string; content: string; content_type: string }[];
    }) => api.uploadIncident(body),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["investigations"] });
      queryClient.invalidateQueries({ queryKey: ["dashboard-stats"] });
    },
  });
}
