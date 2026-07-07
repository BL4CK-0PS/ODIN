"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { UploadDropzone } from "@/components/UploadDropzone";
import { useUploadInvestigation } from "@/hooks/use-investigation";
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

  const { data: stats, isLoading } = useQuery({
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
  });

  const statsConfig = [
    { key: "investigations" as const, label: "Active Investigations" },
    { key: "memories" as const, label: "Memory Objects" },
    { key: "entities" as const, label: "Entities Tracked" },
    { key: "matches" as const, label: "Similarity Matches" },
  ];

  const handleUpload = async (file: File) => {
    const text = await file.text();
    const parsed = JSON.parse(text);
    await upload.mutateAsync({
      title: parsed.title || file.name,
      description: parsed.description || "",
      severity: parsed.severity || "medium",
      evidence: parsed.evidence || [],
    });
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

      <Card>
        <CardHeader>
          <CardTitle>Upload Investigation</CardTitle>
        </CardHeader>
        <CardContent>
          <UploadDropzone onUpload={handleUpload} />
          {upload.isPending && <p className="text-sm text-muted-foreground mt-2">Processing...</p>}
          {upload.isSuccess && <p className="text-sm text-green-400 mt-2">Investigation uploaded successfully.</p>}
          {upload.isError && <p className="text-sm text-red-400 mt-2">Upload failed: {(upload.error as Error).message}</p>}
        </CardContent>
      </Card>
    </div>
  );
}
