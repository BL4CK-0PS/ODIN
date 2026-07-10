"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { UploadDropzone } from "@/components/UploadDropzone";
import { SeverityChart } from "@/components/charts/SeverityChart";
import { EntityTypeChart } from "@/components/charts/EntityTypeChart";
import { useUploadInvestigation } from "@/hooks/use-investigation";
import { useToast } from "@/hooks/use-toast";
import { cn } from "@/lib/utils";
import { Activity, Brain, Shield, AlertTriangle } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { mockStats } from "@/lib/mock-data";

const statIcons = {
  investigations: Activity,
  memories: Brain,
  entities: Shield,
  matches: AlertTriangle,
} as const;

const statStyles = {
  investigations: "bg-blue-500/10 text-blue-500",
  memories: "bg-purple-500/10 text-purple-500",
  entities: "bg-emerald-500/10 text-emerald-500",
  matches: "bg-amber-500/10 text-amber-500",
} as const;

export default function Dashboard() {
  const upload = useUploadInvestigation();
  const { toast } = useToast();

  const { data: stats, isLoading } = useQuery({
    queryKey: ["dashboard-stats"],
    queryFn: async () => {
      try {
        const res = await fetch("/api/v1/system/stats");
        const json = await res.json();
        if (!json.success) throw new Error(json.error || "Failed to fetch stats");
        return json.data as typeof mockStats;
      } catch {
        return mockStats;
      }
    },
    retry: 1,
  });

  const statsConfig = [
    { key: "investigations" as const, label: "Active Investigations" },
    { key: "memories" as const, label: "Memory Objects" },
    { key: "entities" as const, label: "Entities Tracked" },
    { key: "matches" as const, label: "Similarity Matches" },
  ];

  const handleUpload = async (file: File) => {
    try {
      const text = await file.text();

      let parsed: Record<string, unknown>;
      try {
        parsed = JSON.parse(text);
      } catch {
        toast({
          title: "Invalid JSON",
          description: "The file does not contain valid JSON.",
          variant: "error",
        });
        return;
      }

      const result = await upload.mutateAsync({
        title: String(parsed.title || file.name),
        description: String(parsed.description || ""),
        severity: String(parsed.severity || "medium"),
        evidence: (parsed.evidence as any[]) || [],
      });
      toast({
        title: "Investigation uploaded",
        description: `${result.title} (${result.evidence_count} evidence items, ${result.entity_count} entities)`,
        variant: "success",
      });
    } catch (err) {
      toast({
        title: "Upload failed",
        description: err instanceof Error ? err.message : "An unexpected error occurred",
        variant: "error",
      });
    }
  };

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Dashboard</h1>
        <p className="text-muted-foreground mt-1.5">Operational Defense Intelligence Network</p>
      </div>

      <div className="grid grid-cols-4 gap-4">
        {statsConfig.map(({ key, label }) => {
          const Icon = statIcons[key];
          const style = statStyles[key];
          return (
            <Card key={key} className="group hover:shadow-medium transition-all duration-300">
              <CardHeader className="flex-row items-center gap-3 space-y-0 pb-2">
                <div className={cn(
                  "flex items-center justify-center w-10 h-10 rounded-xl transition-transform duration-300 group-hover:scale-110",
                  style
                )}>
                  <Icon className="h-5 w-5" />
                </div>
                <CardTitle className="text-sm font-medium text-muted-foreground">{label}</CardTitle>
              </CardHeader>
              <CardContent>
                {isLoading ? (
                  <Skeleton className="h-9 w-20" />
                ) : (
                  <p className="text-3xl font-bold tracking-tight">{stats?.[key] ?? "—"}</p>
                )}
              </CardContent>
            </Card>
          );
        })}
      </div>

      <div className="grid grid-cols-2 gap-4">
        <SeverityChart />
        <EntityTypeChart />
      </div>

      <Card className="hover:shadow-medium transition-all duration-300">
        <CardHeader>
          <CardTitle>Upload Investigation</CardTitle>
        </CardHeader>
        <CardContent>
          <UploadDropzone onUpload={handleUpload} />
          {upload.isPending && (
            <p className="text-sm text-muted-foreground mt-3 animate-pulse flex items-center gap-2">
              <span className="inline-block w-1.5 h-1.5 rounded-full bg-accent-foreground animate-pulse" />
              Processing investigation...
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
