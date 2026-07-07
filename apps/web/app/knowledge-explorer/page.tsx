"use client";

import { Card, CardContent } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { KnowledgeGraph } from "@/components/KnowledgeGraph";
import { SearchBar } from "@/components/SearchBar";
import { useGlobalGraph } from "@/hooks/use-graph";
import { useGraphStore } from "@/stores/graph";
import { useState } from "react";

export default function KnowledgeExplorerPage() {
  const [query, setQuery] = useState("");
  const { isLoading, error } = useGlobalGraph();
  const nodes = useGraphStore((s) => s.nodes);
  const edges = useGraphStore((s) => s.edges);

  const filteredNodes = query
    ? nodes.filter((n) => n.label.toLowerCase().includes(query.toLowerCase()) || n.type.toLowerCase().includes(query.toLowerCase()))
    : nodes;

  const filteredEdges = query
    ? edges.filter((e) => {
        const source = nodes.find((n) => n.id === e.source);
        const target = nodes.find((n) => n.id === e.target);
        return source && target && filteredNodes.includes(source) && filteredNodes.includes(target);
      })
    : edges;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold">Knowledge Explorer</h1>
        <p className="text-muted-foreground mt-1">Explore entity relationships across all investigations</p>
      </div>

      <SearchBar value={query} onChange={setQuery} placeholder="Filter entities..." />

      {isLoading ? (
        <Card><CardContent className="p-6"><Skeleton className="h-96 w-full" /></CardContent></Card>
      ) : error ? (
        <Card><CardContent className="p-6 text-red-400">Failed to load graph: {(error as Error).message}</CardContent></Card>
      ) : nodes.length === 0 ? (
        <Card><CardContent className="p-6 text-muted-foreground">No entities found. Upload investigations to build the knowledge graph.</CardContent></Card>
      ) : (
        <KnowledgeGraph nodes={filteredNodes} edges={filteredEdges} />
      )}
    </div>
  );
}
