import { useState, useCallback, useEffect } from "react";
import type { Page } from "./types";

export interface Route {
  page: Page;
  params: Record<string, string>;
}

function resolvePath(path: string): Route {
  const parts = path.replace(/^#?\/?/, "").split("/").filter(Boolean);
  if (parts.length === 0) return { page: "dashboard", params: {} };
  if (parts[0] === "investigations" && parts[1]) return { page: "investigation-detail", params: { id: parts[1] } };
  if (parts[0] === "investigations") return { page: "investigations", params: {} };
  if (parts[0] === "threat-memory") return { page: "threat-memory", params: {} };
  if (parts[0] === "knowledge-explorer") return { page: "knowledge-explorer", params: {} };
  if (parts[0] === "search") return { page: "search", params: {} };
  if (parts[0] === "settings") return { page: "settings", params: {} };
  return { page: "dashboard", params: {} };
}

export function navigate(path: string) {
  window.history.pushState({}, "", path);
  window.dispatchEvent(new PopStateEvent("popstate"));
}

export function useRouter(): Route {
  const [route, setRoute] = useState<Route>(() => resolvePath(window.location.pathname));
  const handler = useCallback(() => setRoute(resolvePath(window.location.pathname)), []);
  useEffect(() => {
    window.addEventListener("popstate", handler);
    return () => window.removeEventListener("popstate", handler);
  }, [handler]);
  return route;
}
