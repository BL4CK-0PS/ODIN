import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { UploadDropzone } from "@/components/UploadDropzone";
import { Activity, Brain, Shield, AlertTriangle } from "lucide-react";

const stats = [
  { icon: Activity, label: "Active Investigations", value: "3", color: "text-blue-400" },
  { icon: Brain, label: "Memory Objects", value: "12", color: "text-purple-400" },
  { icon: Shield, label: "Entities Tracked", value: "47", color: "text-green-400" },
  { icon: AlertTriangle, label: "Similarity Matches", value: "8", color: "text-yellow-400" },
];

export function Dashboard() {
  return (
    <div className="space-y-6">
      <div><h1 className="text-3xl font-bold">Dashboard</h1><p className="text-muted-foreground mt-1">Operational Defense Intelligence Network</p></div>
      <div className="grid grid-cols-4 gap-4">
        {stats.map((s) => (
          <Card key={s.label}>
            <CardHeader className="flex-row items-center gap-3 space-y-0">
              <s.icon className={`h-5 w-5 ${s.color}`} />
              <CardTitle className="text-sm font-medium">{s.label}</CardTitle>
            </CardHeader>
            <CardContent><p className="text-3xl font-bold">{s.value}</p></CardContent>
          </Card>
        ))}
      </div>
      <Card>
        <CardHeader><CardTitle>Upload Investigation</CardTitle></CardHeader>
        <CardContent><UploadDropzone onUpload={(f) => console.log("uploaded", f.name)} /></CardContent>
      </Card>
    </div>
  );
}
