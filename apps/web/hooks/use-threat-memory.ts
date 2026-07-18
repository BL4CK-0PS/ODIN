import { useQuery } from "@tanstack/react-query";
import { clientApi, mockApi } from "@/lib/api";
import type { MemoryObject } from "@/lib/types";

const USE_MOCKS = process.env.NEXT_PUBLIC_USE_MOCKS === "true";

export function useThreatMemories() {
  return useQuery({
    queryKey: ["threat-memories"],
    queryFn: async () => {
      if (USE_MOCKS) {
        return (await mockApi.getConsolidationStats(), []) as unknown as MemoryObject[];
      }
      try {
        const res = await fetch("/api/v1/memories");
        const json = await res.json();
        if (!json.success) throw new Error(json.error || "Failed to fetch memories");
        return json.data as MemoryObject[];
      } catch {
        return [];
      }
    },
    retry: 1,
    staleTime: 30_000,
  });
}
