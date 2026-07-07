import { create } from "zustand";
import type { CanonicalIncident, Evidence, Entity } from "@/lib/types";

interface InvestigationState {
  incident: CanonicalIncident | null;
  evidence: Evidence[];
  entities: Entity[];
  loading: boolean;
  setIncident: (incident: CanonicalIncident) => void;
  setEvidence: (evidence: Evidence[]) => void;
  setEntities: (entities: Entity[]) => void;
  setLoading: (loading: boolean) => void;
  reset: () => void;
}

export const useInvestigationStore = create<InvestigationState>((set) => ({
  incident: null,
  evidence: [],
  entities: [],
  loading: false,
  setIncident: (incident) => set({ incident }),
  setEvidence: (evidence) => set({ evidence }),
  setEntities: (entities) => set({ entities }),
  setLoading: (loading) => set({ loading }),
  reset: () => set({ incident: null, evidence: [], entities: [], loading: false }),
}));
