import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Brain, Clock } from "lucide-react";

const memories = [
  { id: "m1", summary: "Ransomware uses PowerShell for C2 communication and registry persistence", confidence: 0.91, version: 3, date: "2026-07-06" },
  { id: "m2", summary: "Phishing campaign targets finance department with credential harvesting", confidence: 0.85, version: 2, date: "2026-07-05" },
  { id: "m3", summary: "Lateral movement via SMB using compromised domain account", confidence: 0.78, version: 1, date: "2026-07-04" },
];

export function ThreatMemory() {
  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <Brain className="h-8 w-8 text-primary" />
        <div><h1 className="text-3xl font-bold">Threat Memory</h1><p className="text-muted-foreground mt-1">Institutional cybersecurity knowledge that compounds over time</p></div>
      </div>
      <div className="grid gap-4">
        {memories.map((m) => (
          <Card key={m.id}>
            <CardHeader className="flex-row items-start justify-between space-y-0">
              <div className="space-y-1">
                <CardTitle className="text-base">{m.summary}</CardTitle>
                <div className="flex items-center gap-2 text-xs text-muted-foreground"><Clock className="h-3 w-3" /><span>{m.date}</span></div>
              </div>
              <div className="flex items-center gap-2"><Badge variant="secondary">v{m.version}</Badge><span className="font-mono text-sm font-medium text-green-400">{(m.confidence * 100).toFixed(0)}%</span></div>
            </CardHeader>
          </Card>
        ))}
      </div>
    </div>
  );
}
