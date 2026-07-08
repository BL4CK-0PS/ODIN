"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { UploadDropzone } from "@/components/UploadDropzone";
import { useUploadInvestigation } from "@/hooks/use-investigation";
import { useToast } from "@/hooks/use-toast";
import { Activity, Brain, Shield, AlertTriangle } from "lucide-react";
import { useQuery } from "@tanstack/react-query";

const statIcons = {
  investigations: Activity,
  memories: Brain,
  entities: Shield,
  matches: AlertTriangle,
} as const;

const statColors = {
  investigations: "text-blue-400",
  memories: "text-purple-400",
  entities: "text-green-400",
  matches: "text-yellow-400",
} as const;

export default function Dashboard() {
  const upload = useUploadInvestigation();
  const { toast } = useToast();

  const { data: stats, isLoading, error: statsError } = useQuery({
    queryKey: ["dashboard-stats"],
    queryFn: async () => {
      const res = await fetch("/api/v1/system/stats");
      const json = await res.json();
      if (!json.success) throw new Error(json.error || "Failed to fetch stats");
      return json.data as {
        investigations: number;
        memories: number;
        entities: number;
        matches: number;
      };
    },
    retry: 2,
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
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold">Dashboard</h1>
        <p className="text-muted-foreground mt-1">Operational Defense Intelligence Network</p>
      </div>

      <div className="grid grid-cols-4 gap-4">
        {statsConfig.map(({ key, label }) => {
          const Icon = statIcons[key];
          return (
            <Card key={key}>
              <CardHeader className="flex-row items-center gap-3 space-y-0">
                <Icon className={`h-5 w-5 ${statColors[key]}`} />
                <CardTitle className="text-sm font-medium">{label}</CardTitle>
              </CardHeader>
              <CardContent>
                {isLoading ? (
                  <Skeleton className="h-9 w-16" />
                ) : (
                  <p className="text-3xl font-bold">{stats?.[key] ?? "—"}</p>
                )}
              </CardContent>
            </Card>
          );
        })}
      </div>

      {statsError && (
        <Card className="border-yellow-400/30">
          <CardContent className="p-4 text-sm text-yellow-400">
            Could not load live stats. API may be unavailable.
          </CardContent>
        </Card>
      )}

      <Card>
        <CardHeader>
          <CardTitle>Upload Investigation</CardTitle>
        </CardHeader>
        <CardContent>
          <UploadDropzone onUpload={handleUpload} />
          {upload.isPending && (
            <p className="text-sm text-muted-foreground mt-2 animate-pulse">
              Processing investigation...
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
