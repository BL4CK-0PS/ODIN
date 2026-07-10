"use client";

import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import { KnowledgeGraph, type GraphLayout } from "@/components/KnowledgeGraph";
import { useGlobalGraph } from "@/hooks/use-graph";
import { useGraphStore } from "@/stores/graph";
import { useState, useMemo } from "react";
import { Globe, X, LayoutGrid, CircleDot, Target, Network, Layers } from "lucide-react";

const NODE_TYPES = ["Incident", "Evidence", "IpAddress", "Domain", "User", "Process", "File"] as const;

const TYPE_COLORS: Record<string, string> = {
  Incident: "bg-blue-500/20 text-blue-400 border-blue-500/30",
  Evidence: "bg-cyan-500/20 text-cyan-400 border-cyan-500/30",
  IpAddress: "bg-emerald-500/20 text-emerald-400 border-emerald-500/30",
  Domain: "bg-emerald-500/20 text-emerald-400 border-emerald-500/30",
  User: "bg-rose-500/20 text-rose-400 border-rose-500/30",
  Process: "bg-purple-500/20 text-purple-400 border-purple-500/30",
  File: "bg-amber-500/20 text-amber-400 border-amber-500/30",
};

const LAYOUTS: { id: GraphLayout; label: string; icon: React.ComponentType<{ className?: string }> }[] = [
  { id: "force", label: "Force", icon: Network },
  { id: "radial", label: "Radial", icon: CircleDot },
  { id: "cluster", label: "Cluster", icon: LayoutGrid },
  { id: "concentric", label: "Concentric", icon: Target },
  { id: "hierarchical", label: "Hierarchical", icon: Layers },
];

export default function KnowledgeExplorerPage() {
  const [query, setQuery] = useState("");
  const [activeFilters, setActiveFilters] = useState<Set<string>>(new Set());
  const [layout, setLayout] = useState<GraphLayout>("force");
  const { isLoading, error } = useGlobalGraph();
  const nodes = useGraphStore((s) => s.nodes);
  const edges = useGraphStore((s) => s.edges);

  const nodeTypeCounts = useMemo(() => {
    const counts: Record<string, number> = {};
    nodes.forEach((n) => { counts[n.type] = (counts[n.type] || 0) + 1; });
    return counts;
  }, [nodes]);

  const toggleFilter = (type: string) => {
    setActiveFilters((prev) => {
      const next = new Set(prev);
      if (next.has(type)) next.delete(type);
      else next.add(type);
      return next;
    });
  };

  const filteredNodes = useMemo(() => {
    let result = nodes;
    if (query) {
      const q = query.toLowerCase();
      result = result.filter((n) =>
        n.label.toLowerCase().includes(q) || n.type.toLowerCase().includes(q),
      );
    }
    if (activeFilters.size > 0) {
      result = result.filter((n) => activeFilters.has(n.type));
    }
    return result;
  }, [nodes, query, activeFilters]);

  const filteredEdges = useMemo(() => {
    if (activeFilters.size === 0 && !query) return edges;
    const validIds = new Set(filteredNodes.map((n) => n.id));
    return edges.filter((e) => validIds.has(e.source) && validIds.has(e.target));
  }, [edges, filteredNodes, activeFilters, query]);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Globe className="h-8 w-8 text-accent-foreground" />
          <div>
            <h1 className="text-3xl font-bold">Knowledge Explorer</h1>
            <p className="text-muted-foreground mt-1">
              {isLoading ? "Loading..." : `${nodes.length} nodes · ${edges.length} edges across all investigations`}
            </p>
          </div>
        </div>
        {filteredNodes.length < nodes.length && (
          <Badge variant="secondary" className="text-xs">
            {filteredNodes.length} of {nodes.length} shown
          </Badge>
        )}
      </div>

      <div className="flex items-center gap-2 flex-wrap">
        {NODE_TYPES.map((type) => {
          const count = nodeTypeCounts[type] || 0;
          const active = activeFilters.has(type);
          return (
            <button
              key={type}
              onClick={() => toggleFilter(type)}
              className={`inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-medium border transition-all ${
                active
                  ? TYPE_COLORS[type] + " ring-1 ring-offset-1 ring-offset-background"
                  : "bg-muted/30 text-muted-foreground border-border/40 hover:border-muted-foreground/30"
              }`}
            >
              {type}
              <span className="opacity-60">({count})</span>
              {active && <X className="h-3 w-3" />}
            </button>
          );
        })}
        {activeFilters.size > 0 && (
          <button
            onClick={() => setActiveFilters(new Set())}
            className="text-xs text-muted-foreground hover:text-foreground underline ml-1"
          >
            Clear filters
          </button>
        )}
      </div>

      <div className="flex items-center gap-4">
        <div className="relative flex-1">
          <input
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Search nodes by name or type..."
            className="flex h-12 w-full rounded-xl border border-border bg-card pl-10 pr-4 text-base shadow-sm placeholder:text-muted-foreground/60 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          />
          <svg className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground pointer-events-none" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
        </div>
        <div className="flex items-center gap-1 p-1 bg-muted/20 border border-border/30 rounded-xl">
          {LAYOUTS.map((l) => {
            const Icon = l.icon;
            const isActive = layout === l.id;
            return (
              <button
                key={l.id}
                onClick={() => setLayout(l.id)}
                className={`inline-flex items-center gap-1.5 px-3 py-2 rounded-lg text-xs font-medium transition-all ${
                  isActive
                    ? "bg-accent text-accent-foreground shadow-sm"
                    : "text-muted-foreground hover:text-foreground hover:bg-muted/30"
                }`}
                title={l.label}
              >
                <Icon className="h-3.5 w-3.5" />
                <span className="hidden sm:inline">{l.label}</span>
              </button>
            );
          })}
        </div>
      </div>

      {isLoading ? (
        <Card><CardContent className="p-6"><Skeleton className="h-96 w-full" /></CardContent></Card>
      ) : error ? (
        <Card><CardContent className="p-6 text-red-400 flex items-center gap-2">
          <span className="w-2 h-2 rounded-full bg-red-400 animate-pulse" />
          Failed to load graph: {(error as Error).message}
        </CardContent></Card>
      ) : nodes.length === 0 ? (
        <Card>
          <CardContent className="p-12 text-center text-muted-foreground space-y-2">
            <Globe className="h-12 w-12 mx-auto opacity-20" />
            <p>No entities found.</p>
            <p className="text-sm">Upload investigations from the Dashboard to build the knowledge graph.</p>
          </CardContent>
        </Card>
      ) : (
        <KnowledgeGraph
          nodes={filteredNodes}
          edges={filteredEdges}
          allNodeCount={nodes.length}
          layout={layout}
        />
      )}
    </div>
  );
}
