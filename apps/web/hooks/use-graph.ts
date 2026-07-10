import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";
import { useGraphStore } from "@/stores/graph";
import { mockGraph } from "@/lib/mock-data";
import type { GraphData, GraphNode, GraphEdge } from "@/stores/graph";

export function useInvestigationGraph(incidentId: string | null) {
  const setNodes = useGraphStore((s) => s.setNodes);
  const setEdges = useGraphStore((s) => s.setEdges);

  return useQuery({
    queryKey: ["graph", incidentId],
    queryFn: async () => {
      try {
        const data = await api.getGraph(incidentId!);
        setNodes(data.nodes as GraphNode[]);
        setEdges(data.edges as GraphEdge[]);
        return data;
      } catch {
        setNodes(mockGraph.nodes);
        setEdges(mockGraph.edges);
        return mockGraph;
      }
    },
    enabled: !!incidentId,
    retry: 1,
    staleTime: 30_000,
  });
}

export function useGlobalGraph() {
  const setNodes = useGraphStore((s) => s.setNodes);
  const setEdges = useGraphStore((s) => s.setEdges);

  return useQuery({
    queryKey: ["global-graph"],
    queryFn: async () => {
      try {
        const res = await fetch("/api/v1/graph");
        const json = await res.json();
        if (!json.success) throw new Error(json.error || "Failed to fetch graph");
        const data = json.data as GraphData;
        setNodes(data.nodes);
        setEdges(data.edges);
        return data;
      } catch {
        setNodes(mockGraph.nodes);
        setEdges(mockGraph.edges);
        return mockGraph;
      }
    },
    retry: 1,
    staleTime: 30_000,
  });
}
