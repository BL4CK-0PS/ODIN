import { useQuery } from "@tanstack/react-query";
import { mockThreatMemories } from "@/lib/mock-data";
import type { MemoryObject } from "@/lib/types";

export function useThreatMemories() {
  return useQuery({
    queryKey: ["threat-memories"],
    queryFn: async () => {
      try {
        const res = await fetch("/api/v1/memories");
        const json = await res.json();
        if (!json.success) throw new Error(json.error || "Failed to fetch memories");
        return json.data as MemoryObject[];
      } catch {
        return mockThreatMemories;
      }
    },
    retry: 1,
    staleTime: 30_000,
  });
}
