"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { Badge } from "@/components/ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { ConfidenceHistogram } from "@/components/charts/ConfidenceHistogram";
import { DiffCard } from "@/components/DiffCard";
import { useThreatMemories } from "@/hooks/use-threat-memory";
import { useQuery } from "@tanstack/react-query";
import { clientApi } from "@/lib/api";
import { Archive, Trash2, GitCompare, TimerOff, AlertCircle, Activity } from "lucide-react";

function formatRelativeTime(iso: string): string {
  const diff = Date.now() - new Date(iso).getTime();
  const mins = Math.floor(diff / 60_000);
  if (mins < 1) return "Just now";
  if (mins < 60) return `${mins}m ago`;
  const hours = Math.floor(mins / 60);
  if (hours < 24) return `${hours}h ago`;
  const days = Math.floor(hours / 24);
  return `${days}d ago`;
}

export default function ConsolidationPage() {
  const { data: memories, isLoading: memoriesLoading } = useThreatMemories();

  const { data: stats, isLoading: statsLoading } = useQuery({
    queryKey: ["consolidation-stats"],
    queryFn: () => clientApi.getConsolidationStats(),
    retry: 1,
    staleTime: 30_000,
  });

  const isLoading = memoriesLoading || statsLoading;

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
          { label: "Total Memories", value: stats?.total_memories ?? memories?.length ?? 0, icon: Activity, style: "bg-blue-500/10 text-blue-500" },
          { label: "Expired Purged", value: stats?.expired_purged ?? 0, icon: Trash2, style: "bg-red-500/10 text-red-500" },
          { label: "Versions Pruned", value: stats?.versions_pruned ?? 0, icon: TimerOff, style: "bg-amber-500/10 text-amber-500" },
          { label: "Consolidated", value: stats?.memories_consolidated ?? 0, icon: GitCompare, style: "bg-green-500/10 text-green-500" },
          {
            label: "Last Run",
            value: stats?.last_consolidation
              ? formatRelativeTime(stats.last_consolidation)
              : "Never",
            icon: Archive,
            style: "bg-purple-500/10 text-purple-500"
          },
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
          {memories && memories.filter(m => m.version > 1).length > 0 ? (
            memories.filter(m => m.version > 1).map((m) => (
              <DiffCard
                key={m.id}
                title={`${m.id} → v${m.version}`}
                current={`[v${m.version}] ${m.summary.slice(0, 100)}...`}
                previous={`[v${m.version - 1}] Pre-consolidation state`}
                type="changed"
              />
            ))
          ) : memories && memories.length > 0 ? (
            <Card>
              <CardHeader>
                <CardTitle className="text-sm">Memory Version Summary</CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-sm text-muted-foreground">
                  {memories.length} memories exist, each retaining up to 10 versions.
                  {stats?.versions_pruned ?? 0} old versions were pruned in the last consolidation cycle.
                </p>
              </CardContent>
            </Card>
          ) : !isLoading ? (
            <Card>
              <CardContent className="p-6 text-muted-foreground flex items-center gap-2">
                <AlertCircle className="h-4 w-4" />
                No memories yet. They are created as investigations are processed.
              </CardContent>
            </Card>
          ) : null}
        </TabsContent>

        <TabsContent value="ttl" className="space-y-4 mt-4">
          <Card>
            <CardHeader>
              <CardTitle className="text-sm">Time-to-Live by Severity</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                {Object.entries(stats?.ttl_config ?? { critical: "365 days", high: "180 days", medium: "90 days", low: "30 days" }).map(([severity, ttl]) => (
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
