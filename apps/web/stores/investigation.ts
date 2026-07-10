import { create } from "zustand";
import type { CanonicalIncident } from "@/lib/types";

interface InvestigationState {
  incident: CanonicalIncident | null;
  setIncident: (incident: CanonicalIncident) => void;
  reset: () => void;
}

export const useInvestigationStore = create<InvestigationState>((set) => ({
  incident: null,
  setIncident: (incident) => set({ incident }),
  reset: () => set({ incident: null }),
}));
