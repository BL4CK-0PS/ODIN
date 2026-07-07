"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import Link from "next/link";

const mockIncidents = [
  { id: "inc-1", title: "Ransomware Outbreak — Finance Department", severity: "Critical", status: "Investigating", techniques: ["T1059", "T1486"], date: "2026-07-06" },
  { id: "inc-2", title: "Phishing Campaign — Credential Harvesting", severity: "High", status: "Contained", techniques: ["T1566", "T1078"], date: "2026-07-05" },
  { id: "inc-3", title: "Lateral Movement — Unauthorized SMB Access", severity: "Medium", status: "New", techniques: ["T1021"], date: "2026-07-04" },
];

const sevColor: Record<string, string> = {
  Critical: "text-red-400 border-red-400/30",
  High: "text-orange-400 border-orange-400/30",
  Medium: "text-yellow-400 border-yellow-400/30",
  Low: "text-green-400 border-green-400/30",
};

export default function InvestigationsPage() {
  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Investigations</h1>
      <div className="grid gap-4">
        {mockIncidents.map((inc) => (
          <Link key={inc.id} href={`/investigations/${inc.id}`}>
            <Card className="cursor-pointer transition-colors hover:border-primary/50">
              <CardHeader className="flex-row items-start justify-between space-y-0">
                <div>
                  <CardTitle className="text-base">{inc.title}</CardTitle>
                  <p className="text-xs text-muted-foreground mt-1">{inc.date}</p>
                </div>
                <Badge variant="outline" className={sevColor[inc.severity]}>
                  {inc.severity}
                </Badge>
              </CardHeader>
              <CardContent className="flex gap-2">
                <Badge variant="secondary">{inc.status}</Badge>
                {inc.techniques.map((t) => (
                  <Badge key={t} variant="outline">{t}</Badge>
                ))}
              </CardContent>
            </Card>
          </Link>
        ))}
      </div>
    </div>
  );
}
