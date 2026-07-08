"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { FileText, ShieldAlert } from "lucide-react";

interface NarrativeCardProps {
  summary: string;
  confidence: number;
  techniques: string[];
}

export function NarrativeCard({ summary, confidence, techniques }: NarrativeCardProps) {
  // Highlight IOCs and MITRE techniques in the summary text dynamically
  const highlightSummary = (text: string) => {
    if (!text) return "";
    const regex = /(\bT\d{4}\b|\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b|\b[a-zA-Z0-9.-]+\.(?:com|net|org|xyz|local)\b|\b[a-zA-Z0-9.-]+\.(?:exe|dll|ps1|bat)\b)/gi;
    const parts = text.split(regex);
    
    return parts.map((part, i) => {
      if (part.match(regex)) {
        return (
          <span key={i} className="font-mono text-xs font-semibold px-1.5 py-0.5 mx-0.5 rounded bg-primary/10 border border-primary/25 text-primary-foreground text-blue-400">
            {part}
          </span>
        );
      }
      return part;
    });
  };

  const isHighConfidence = confidence >= 0.8;

  return (
    <Card className="glass relative overflow-hidden flex flex-col h-full">
      <CardHeader className="pb-3 border-b border-border/40">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <FileText className="h-5 w-5 text-primary" />
            <CardTitle className="text-lg font-semibold tracking-tight">Attack Narrative</CardTitle>
          </div>
          {isHighConfidence && (
            <Badge variant="outline" className="border-emerald-500/30 text-emerald-400 bg-emerald-500/5 text-[10px] font-mono flex items-center gap-1">
              <ShieldAlert className="h-3 w-3" />
              Verified Memory
            </Badge>
          )}
        </div>
      </CardHeader>
      
      <CardContent className="flex-1 flex flex-col justify-between pt-4 gap-4">
        {/* Dynamic Summary with highlighted parts */}
        <p className="text-sm leading-relaxed text-muted-foreground break-words">
          {highlightSummary(summary)}
        </p>

        <div className="space-y-3.5 pt-2 border-t border-border/40">
          <div className="flex items-center justify-between">
            <span className="text-xs text-muted-foreground font-medium">Pipeline Confidence:</span>
            <span className={`text-sm font-mono font-bold ${isHighConfidence ? "text-green-400" : "text-yellow-400"}`}>
              {(confidence * 100).toFixed(0)}%
            </span>
          </div>
          {/* Progress bar */}
          <div className="w-full h-1 bg-border/40 rounded-full overflow-hidden">
            <div
              className={`h-full transition-all duration-1000 ${isHighConfidence ? "bg-green-400" : "bg-yellow-400"}`}
              style={{ width: `${confidence * 100}%` }}
            />
          </div>

          <div className="flex flex-wrap gap-1.5 pt-1">
            {techniques.map((t) => (
              <Badge
                key={t}
                variant="secondary"
                className="text-[10px] font-mono py-0.5 px-2 bg-secondary border-border/60 hover:bg-secondary/80 text-foreground transition-all duration-200"
              >
                {t}
              </Badge>
            ))}
            {techniques.length === 0 && (
              <span className="text-xs text-muted-foreground italic">No MITRE ATT&CK techniques mapped.</span>
            )}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
