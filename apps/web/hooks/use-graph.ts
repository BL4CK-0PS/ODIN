import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";
import { useGraphStore } from "@/stores/graph";

export function useInvestigationGraph(incidentId: string | null) {
  const setNodes = useGraphStore((s) => s.setNodes);
  const setEdges = useGraphStore((s) => s.setEdges);

  return useQuery({
    queryKey: ["graph", incidentId],
    queryFn: async () => {
      const data = await api.getGraph(incidentId!);
      setNodes(data.nodes as any);
      setEdges(data.edges as any);
      return data;
    },
    enabled: !!incidentId,
  });
}

export function useGlobalGraph() {
  const setNodes = useGraphStore((s) => s.setNodes);
  const setEdges = useGraphStore((s) => s.setEdges);

  return useQuery({
    queryKey: ["global-graph"],
    queryFn: async () => {
      const res = await fetch("/api/v1/graph");
      const json = await res.json();
      if (!json.success) throw new Error(json.error || "Failed to fetch graph");
      setNodes(json.data.nodes as any);
      setEdges(json.data.edges as any);
      return json.data;
    },
  });
}
