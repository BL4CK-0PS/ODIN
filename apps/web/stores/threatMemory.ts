import { create } from "zustand";
import type { MemoryObject } from "@/lib/types";

interface ThreatMemoryState {
  memories: MemoryObject[];
  selected: MemoryObject | null;
  loading: boolean;
  setMemories: (memories: MemoryObject[]) => void;
  setSelected: (memory: MemoryObject | null) => void;
  setLoading: (loading: boolean) => void;
}

export const useThreatMemoryStore = create<ThreatMemoryState>((set) => ({
  memories: [],
  selected: null,
  loading: false,
  setMemories: (memories) => set({ memories }),
  setSelected: (selected) => set({ selected }),
  setLoading: (loading) => set({ loading }),
}));
