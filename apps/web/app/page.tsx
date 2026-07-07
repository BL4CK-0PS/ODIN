"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { UploadDropzone } from "@/components/UploadDropzone";
import { api } from "@/lib/api";
import { useState } from "react";
import { Activity, Brain, Shield, AlertTriangle } from "lucide-react";

export default function Dashboard() {
  const [uploading, setUploading] = useState(false);

  const handleUpload = async (file: File) => {
    setUploading(true);
    const text = await file.text();
    const parsed = JSON.parse(text);
    await api.uploadIncident({
      title: parsed.title || file.name,
      description: parsed.description || "",
      severity: parsed.severity || "medium",
      evidence: parsed.evidence || [],
    });
    setUploading(false);
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold">Dashboard</h1>
        <p className="text-muted-foreground mt-1">Operational Defense Intelligence Network</p>
      </div>

      <div className="grid grid-cols-4 gap-4">
        {[
          { icon: Activity, label: "Active Investigations", value: "3", color: "text-blue-400" },
          { icon: Brain, label: "Memory Objects", value: "12", color: "text-purple-400" },
          { icon: Shield, label: "Entities Tracked", value: "47", color: "text-green-400" },
          { icon: AlertTriangle, label: "Similarity Matches", value: "8", color: "text-yellow-400" },
        ].map((stat) => (
          <Card key={stat.label}>
            <CardHeader className="flex-row items-center gap-3 space-y-0">
              <stat.icon className={`h-5 w-5 ${stat.color}`} />
              <CardTitle className="text-sm font-medium">{stat.label}</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-3xl font-bold">{stat.value}</p>
            </CardContent>
          </Card>
        ))}
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Upload Investigation</CardTitle>
        </CardHeader>
        <CardContent>
          <UploadDropzone onUpload={handleUpload} />
          {uploading && <p className="text-sm text-muted-foreground mt-2">Processing...</p>}
        </CardContent>
      </Card>
    </div>
  );
}
