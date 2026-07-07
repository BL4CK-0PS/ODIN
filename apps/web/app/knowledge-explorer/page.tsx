"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { KnowledgeGraph } from "@/components/KnowledgeGraph";
import { SearchBar } from "@/components/SearchBar";
import { useState } from "react";

const mockNodes = [
  { id: "n1", type: "Process", label: "powershell.exe" },
  { id: "n2", type: "Network", label: "evil-c2.com" },
  { id: "n3", type: "Hash", label: "a1b2c3d4e5f6..." },
  { id: "n4", type: "Registry", label: "HKLM\\SOFTWARE\\Malware" },
  { id: "n5", type: "Domain", label: "phish.bad.com" },
];

const mockEdges = [
  { source: "n1", target: "n2", label: "connected_to" },
  { source: "n1", target: "n3", label: "created" },
  { source: "n1", target: "n4", label: "modified" },
  { source: "n5", target: "n2", label: "resolves_to" },
];

export default function KnowledgeExplorerPage() {
  const [query, setQuery] = useState("");

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold">Knowledge Explorer</h1>
        <p className="text-muted-foreground mt-1">Explore entity relationships across all investigations</p>
      </div>

      <SearchBar value={query} onChange={setQuery} placeholder="Filter entities..." />

      <KnowledgeGraph nodes={mockNodes} edges={mockEdges} />
    </div>
  );
}
