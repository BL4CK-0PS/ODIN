"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { Badge } from "@/components/ui/badge";
import { useThreatMemories } from "@/hooks/use-threat-memory";
import { Brain, Clock, AlertCircle } from "lucide-react";

export default function ThreatMemoryPage() {
  const { data: memories, isLoading, error } = useThreatMemories();

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <Brain className="h-8 w-8 text-primary" />
        <div>
          <h1 className="text-3xl font-bold">Threat Memory</h1>
          <p className="text-muted-foreground mt-1">Institutional cybersecurity knowledge that compounds over time</p>
        </div>
      </div>

      <div className="grid gap-4">
        {isLoading ? (
          [1, 2, 3].map((i) => (
            <Card key={i}>
              <CardHeader className="flex-row items-start justify-between space-y-0">
                <div className="space-y-2">
                  <Skeleton className="h-5 w-96" />
                  <Skeleton className="h-3 w-24" />
                </div>
                <div className="flex items-center gap-2">
                  <Skeleton className="h-5 w-10" />
                  <Skeleton className="h-5 w-12" />
                </div>
              </CardHeader>
            </Card>
          ))
        ) : error ? (
          <Card>
            <CardContent className="p-6 text-red-400 flex items-center gap-2">
              <AlertCircle className="h-4 w-4" />
              Failed to load memories: {(error as Error).message}
            </CardContent>
          </Card>
        ) : (memories?.length ?? 0) === 0 ? (
          <Card>
            <CardContent className="p-6 text-muted-foreground">
              No threat memories yet. They are created as investigations are processed.
            </CardContent>
          </Card>
        ) : (
          memories?.map((m) => (
            <Card key={m.id}>
              <CardHeader className="flex-row items-start justify-between space-y-0">
                <div className="space-y-1">
                  <CardTitle className="text-base">{m.summary}</CardTitle>
                  <div className="flex items-center gap-2 text-xs text-muted-foreground">
                    <Clock className="h-3 w-3" />
                    <span>{m.created_at ? new Date(m.created_at).toLocaleDateString() : ""}</span>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <Badge variant="secondary">v{m.version ?? 1}</Badge>
                  <span className="font-mono text-sm font-medium text-green-400">
                    {((m.confidence ?? 0) * 100).toFixed(0)}%
                  </span>
                </div>
              </CardHeader>
            </Card>
          ))
        )}
      </div>
    </div>
  );
}
