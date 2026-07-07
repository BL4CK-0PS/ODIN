import { create } from "zustand";
import type { RankedResult } from "@/lib/types";

interface SearchState {
  query: string;
  results: RankedResult[];
  loading: boolean;
  setQuery: (query: string) => void;
  setResults: (results: RankedResult[]) => void;
  setLoading: (loading: boolean) => void;
}

export const useSearchStore = create<SearchState>((set) => ({
  query: "",
  results: [],
  loading: false,
  setQuery: (query) => set({ query }),
  setResults: (results) => set({ results }),
  setLoading: (loading) => set({ loading }),
}));
