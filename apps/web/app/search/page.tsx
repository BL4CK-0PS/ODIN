"use client";

import { SearchBar } from "@/components/SearchBar";
import { SimilarityCard } from "@/components/SimilarityCard";
import { useState } from "react";

const mockResults = [
  { title: "Ransomware — HR Dept (2026-05)", score: 0.92, reasons: ["Same T1059", "Same PowerShell", "Same Registry Key"] },
  { title: "Phishing — Engineering (2026-04)", score: 0.67, reasons: ["Same T1566", "Different C2"] },
  { title: "Data Exfil — Marketing (2026-03)", score: 0.45, reasons: ["No technique overlap", "Different TTPs"] },
];

export default function SearchPage() {
  const [query, setQuery] = useState("");

  return (
    <div className="space-y-6 max-w-3xl">
      <div>
        <h1 className="text-3xl font-bold">Search</h1>
        <p className="text-muted-foreground mt-1">Find similar investigations across institutional memory</p>
      </div>

      <SearchBar value={query} onChange={setQuery} />

      <div className="space-y-3">
        {mockResults.map((r, i) => (
          <SimilarityCard key={i} title={r.title} score={r.score} reasons={r.reasons} />
        ))}
      </div>
    </div>
  );
}
