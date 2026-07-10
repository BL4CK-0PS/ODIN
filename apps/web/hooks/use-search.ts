import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";
import { getMockSimilarResults } from "@/lib/mock-data";
import type { RankedResult } from "@/lib/types";

export function useSearchSimilar(incidentId: string | null, topK = 5) {
  return useQuery({
    queryKey: ["search-similar", incidentId, topK],
    queryFn: async () => {
      try {
        const res = await api.searchSimilar(incidentId!, topK);
        return res.results as RankedResult[];
      } catch {
        return getMockSimilarResults(incidentId!);
      }
    },
    enabled: !!incidentId,
    retry: 1,
    staleTime: 30_000,
  });
}

export function useSearchQuery(query: string) {
  return useQuery({
    queryKey: ["search", query],
    queryFn: async () => {
      try {
        const res = await fetch("/api/v1/search", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ query, top_k: 10 }),
        });
        const json = await res.json();
        if (!json.success) throw new Error(json.error || "Search failed");
        return json.data.results as RankedResult[];
      } catch {
        return getMockSimilarResults("inc-001");
      }
    },
    enabled: query.length > 2,
    retry: 1,
    staleTime: 15_000,
  });
}
