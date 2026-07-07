import { create } from "zustand";

interface SettingsState {
  apiUrl: string;
  theme: "dark" | "light";
  setApiUrl: (url: string) => void;
  setTheme: (theme: "dark" | "light") => void;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  apiUrl: process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000/api/v1",
  theme: "dark",
  setApiUrl: (apiUrl) => set({ apiUrl }),
  setTheme: (theme) => set({ theme }),
}));
