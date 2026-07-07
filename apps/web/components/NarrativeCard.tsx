"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { FileText } from "lucide-react";

interface NarrativeCardProps {
  summary: string;
  confidence: number;
  techniques: string[];
}

export function NarrativeCard({ summary, confidence, techniques }: NarrativeCardProps) {
  return (
    <Card>
      <CardHeader>
        <div className="flex items-center gap-2">
          <FileText className="h-5 w-5 text-primary" />
          <CardTitle className="text-lg">Attack Narrative</CardTitle>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        <p className="text-sm leading-relaxed text-muted-foreground">{summary}</p>
        <div className="flex items-center gap-2">
          <span className="text-xs text-muted-foreground">Confidence:</span>
          <span className="text-sm font-mono font-medium">
            {(confidence * 100).toFixed(0)}%
          </span>
        </div>
        <div className="flex flex-wrap gap-1.5">
          {techniques.map((t) => (
            <Badge key={t} variant="secondary">{t}</Badge>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}
