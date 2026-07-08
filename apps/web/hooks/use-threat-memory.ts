import { useQuery } from "@tanstack/react-query";
import type { MemoryObject } from "@/lib/types";

export function useThreatMemories() {
  return useQuery({
    queryKey: ["threat-memories"],
    queryFn: async () => {
      const res = await fetch("/api/v1/memories");
      const json = await res.json();
      if (!json.success) throw new Error(json.error || "Failed to fetch memories");
      return json.data as MemoryObject[];
    },
    retry: 2,
    staleTime: 30_000,
  });
}
