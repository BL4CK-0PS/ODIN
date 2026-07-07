import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api";
import { useInvestigationStore } from "@/stores/investigation";

export function useInvestigation(id: string) {
  const setIncident = useInvestigationStore((s) => s.setIncident);
  const setEvidence = useInvestigationStore((s) => s.setEvidence);
  const setEntities = useInvestigationStore((s) => s.setEntities);

  return useQuery({
    queryKey: ["investigation", id],
    queryFn: async () => {
      const [incident, timeline, memory, graph, playbooks] = await Promise.all([
        api.getIncident(id),
        api.getTimeline(id),
        api.getMemory(id),
        api.getGraph(id),
        api.getPlaybooks(id),
      ]);
      setIncident(incident as any);
      setEvidence((incident as any).evidence || []);
      setEntities((incident as any).entities || []);
      return { incident, timeline, memory, graph, playbooks };
    },
    enabled: !!id,
  });
}

export function useInvestigations() {
  return useQuery({
    queryKey: ["investigations"],
    queryFn: async () => {
      const res = await fetch("/api/v1/incidents");
      const json = await res.json();
      if (!json.success) throw new Error(json.error || "Failed to fetch investigations");
      return json.data as any[];
    },
  });
}

export function useUploadInvestigation() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: { title: string; description: string; severity: string; evidence: { source: string; content: string; content_type: string }[] }) =>
      api.uploadIncident(body),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["investigations"] });
    },
  });
}
