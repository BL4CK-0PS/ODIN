import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";
import type { RankedResult } from "@/lib/types";

export function useSearchSimilar(incidentId: string | null, topK = 5) {
  return useQuery({
    queryKey: ["search-similar", incidentId, topK],
    queryFn: async () => {
      const res = await api.searchSimilar(incidentId!, topK);
      return res.results as RankedResult[];
    },
    enabled: !!incidentId,
  });
}

export function useSearchQuery(query: string) {
  return useQuery({
    queryKey: ["search", query],
    queryFn: async () => {
      const res = await fetch("/api/v1/search", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ query, top_k: 10 }),
      });
      const json = await res.json();
      if (!json.success) throw new Error(json.error || "Search failed");
      return json.data.results as RankedResult[];
    },
    enabled: query.length > 2,
  });
}
