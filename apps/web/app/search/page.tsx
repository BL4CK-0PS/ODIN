"use client";

import { SearchBar } from "@/components/SearchBar";
import { SimilarityCard } from "@/components/SimilarityCard";
import { SimilarityReason } from "@/components/SimilarityReason";
import { Skeleton } from "@/components/ui/skeleton";
import { Card, CardContent } from "@/components/ui/card";
import { useSearchQuery } from "@/hooks/use-search";
import { useState } from "react";
import { AlertCircle } from "lucide-react";
import type { RankedResult } from "@/lib/types";

export default function SearchPage() {
  const [query, setQuery] = useState("");
  const { data: results, isLoading, error } = useSearchQuery(query);

  return (
    <div className="space-y-6 max-w-3xl">
      <div>
        <h1 className="text-3xl font-bold">Search</h1>
        <p className="text-muted-foreground mt-1">Find similar investigations across institutional memory</p>
      </div>

      <SearchBar value={query} onChange={setQuery} />

      {query.length > 0 && query.length <= 2 && (
        <p className="text-sm text-muted-foreground">Enter at least 3 characters to search.</p>
      )}

      {isLoading && (
        <div className="space-y-3">
          {[1, 2].map((i) => (
            <Card key={i}>
              <CardContent className="p-4"><Skeleton className="h-16 w-full" /></CardContent>
            </Card>
          ))}
        </div>
      )}

      {error && (
        <Card>
          <CardContent className="p-4 text-red-400 flex items-center gap-2">
            <AlertCircle className="h-4 w-4" />
            Search failed: {(error as Error).message}
          </CardContent>
        </Card>
      )}

      {results && results.length === 0 && query.length > 2 && (
        <Card>
          <CardContent className="p-4 text-muted-foreground">
            No results found for &ldquo;{query}&rdquo;. Try different keywords.
          </CardContent>
        </Card>
      )}

      <div className="space-y-3">
        {results?.map((r: RankedResult, i: number) => (
          <div key={i} className="space-y-2">
            <SimilarityCard
              title={r.memory?.summary || `Result ${i + 1}`}
              score={r.score?.overall ?? 0}
              reasons={r.reasons || []}
            />
            {r.reasons && r.reasons.length > 0 && (
              <SimilarityReason
                reasons={r.reasons.map((reason: string) => ({
                  label: reason,
                  matched: true,
                }))}
              />
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
