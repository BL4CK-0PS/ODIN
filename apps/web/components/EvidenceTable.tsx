"use client";

import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { cn } from "@/lib/utils";
import { Scale } from "lucide-react";

interface EvidenceItem {
  id: string;
  source: string;
  content_type: string;
  trust_score: number;
}

interface EvidenceTableProps {
  evidence: EvidenceItem[];
}

export function EvidenceTable({ evidence }: EvidenceTableProps) {
  const trustColor = (s: number) => s >= 0.8 ? "text-green-400" : s >= 0.5 ? "text-yellow-400" : "text-red-400";

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center gap-2">
          <Scale className="h-5 w-5 text-primary" />
          <CardTitle className="text-lg">Evidence</CardTitle>
        </div>
      </CardHeader>
      <CardContent>
        <ScrollArea className="max-h-80">
          <div className="space-y-2">
            {evidence.map((e) => (
              <div key={e.id} className="flex items-center justify-between p-3 rounded-lg bg-secondary/50">
                <div>
                  <p className="text-sm font-medium">{e.source}</p>
                  <p className="text-xs text-muted-foreground">{e.content_type}</p>
                </div>
                <span className={cn("font-mono text-sm font-medium", trustColor(e.trust_score))}>
                  {(e.trust_score * 100).toFixed(0)}
                </span>
              </div>
            ))}
          </div>
        </ScrollArea>
      </CardContent>
    </Card>
  );
}
