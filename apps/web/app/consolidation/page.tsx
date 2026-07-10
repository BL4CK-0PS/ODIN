"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { Badge } from "@/components/ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { ConfidenceHistogram } from "@/components/charts/ConfidenceHistogram";
import { DiffCard } from "@/components/DiffCard";
import { useThreatMemories } from "@/hooks/use-threat-memory";
import { useQuery } from "@tanstack/react-query";
import { Archive, Trash2, GitCompare, TimerOff, AlertCircle, Activity } from "lucide-react";

const mockConsolidation = {
  last_run: "2026-07-10T06:00:00Z",
  expired_purged: 12,
  versions_pruned: 47,
  memories_consolidated: 5,
  total_memories: 89,
  ttl_config: {
    critical: "365 days",
    high: "180 days",
    medium: "90 days",
    low: "30 days",
  },
  recent_versions: [
    { memory_id: "mem-001", version: 3, changelog: "Consolidated by summarization agent", created_at: "2026-07-10T06:00:00Z" },
    { memory_id: "mem-002", version: 2, changelog: "Confidence adjusted from feedback", created_at: "2026-07-10T05:45:00Z" },
    { memory_id: "mem-003", version: 4, changelog: "Consolidated by summarization agent", created_at: "2026-07-10T05:30:00Z" },
  ],
};

export default function ConsolidationPage() {
  const { data: memories, isLoading: memoriesLoading } = useThreatMemories();

  const { data: stats } = useQuery({
    queryKey: ["dashboard-stats"],
    queryFn: async () => {
      try {
        const res = await fetch("/api/v1/system/stats");
        const json = await res.json();
        return json.success ? json.data : null;
      } catch { return null; }
    },
    retry: 1,
  });

  const isLoading = memoriesLoading;

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <Archive className="h-8 w-8 text-accent-foreground" />
        <div>
          <h1 className="text-3xl font-bold">Consolidation</h1>
          <p className="text-muted-foreground mt-1">Memory lifecycle management — TTL expiry, version pruning, and AI summarization</p>
        </div>
      </div>

      <div className="grid grid-cols-5 gap-4">
        {[
          { label: "Total Memories", value: stats?.memories ?? mockConsolidation.total_memories, icon: Activity, style: "bg-blue-500/10 text-blue-500" },
          { label: "Expired Purged", value: mockConsolidation.expired_purged, icon: Trash2, style: "bg-red-500/10 text-red-500" },
          { label: "Versions Pruned", value: mockConsolidation.versions_pruned, icon: TimerOff, style: "bg-amber-500/10 text-amber-500" },
          { label: "Consolidated", value: mockConsolidation.memories_consolidated, icon: GitCompare, style: "bg-green-500/10 text-green-500" },
          { label: "Last Run", value: "6h ago", icon: Archive, style: "bg-purple-500/10 text-purple-500" },
        ].map(({ label, value, icon: Icon, style }) => (
          <Card key={label}>
            <CardHeader className="flex-row items-center gap-3 space-y-0 pb-2">
              <div className={`flex items-center justify-center w-10 h-10 rounded-xl ${style}`}>
                <Icon className="h-5 w-5" />
              </div>
              <CardTitle className="text-sm font-medium text-muted-foreground">{label}</CardTitle>
            </CardHeader>
            <CardContent>
              {isLoading ? (
                <Skeleton className="h-9 w-16" />
              ) : (
                <p className="text-3xl font-bold tracking-tight">{value}</p>
              )}
            </CardContent>
          </Card>
        ))}
      </div>

      <Tabs defaultValue="versions">
        <TabsList>
          <TabsTrigger value="versions">Version Changes</TabsTrigger>
          <TabsTrigger value="ttl">TTL Configuration</TabsTrigger>
          <TabsTrigger value="distribution">Confidence Distribution</TabsTrigger>
        </TabsList>

        <TabsContent value="versions" className="space-y-4 mt-4">
          {mockConsolidation.recent_versions.map((v) => (
            <DiffCard
              key={`${v.memory_id}-${v.version}`}
              title={`${v.memory_id} → v${v.version}`}
              current={`[v${v.version}] ${v.changelog}`}
              previous={`[v${v.version - 1}] Pre-consolidation state`}
              type="changed"
            />
          ))}
          {memories && memories.length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle className="text-sm">Memory Version Summary</CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-sm text-muted-foreground">
                  {memories.length} memories exist, each retaining up to 10 versions. 
                  {mockConsolidation.versions_pruned} old versions were pruned in the last consolidation cycle.
                </p>
              </CardContent>
            </Card>
          )}
          {!isLoading && (!memories || memories.length === 0) && (
            <Card>
              <CardContent className="p-6 text-muted-foreground flex items-center gap-2">
                <AlertCircle className="h-4 w-4" />
                No memories yet. They are created as investigations are processed.
              </CardContent>
            </Card>
          )}
        </TabsContent>

        <TabsContent value="ttl" className="space-y-4 mt-4">
          <Card>
            <CardHeader>
              <CardTitle className="text-sm">Time-to-Live by Severity</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                {Object.entries(mockConsolidation.ttl_config).map(([severity, ttl]) => (
                  <div key={severity} className="flex items-center justify-between py-2 border-b border-border last:border-0">
                    <span className="text-sm font-medium capitalize">{severity}</span>
                    <Badge variant="secondary">{ttl}</Badge>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle className="text-sm">Pruning Policy</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-muted-foreground">
                Each memory retains a maximum of 10 versions. When exceeded, the oldest versions are automatically pruned during the consolidation cycle.
              </p>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="distribution" className="mt-4">
          {memories && memories.length > 0 ? (
            <ConfidenceHistogram memories={memories} />
          ) : isLoading ? (
            <Skeleton className="h-48 w-full" />
          ) : (
            <Card>
              <CardContent className="p-6 text-muted-foreground">No data available.</CardContent>
            </Card>
          )}
        </TabsContent>
      </Tabs>
    </div>
  );
}
