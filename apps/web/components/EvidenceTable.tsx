"use client";

import { useState } from "react";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { cn } from "@/lib/utils";
import { Scale, Eye, FileCode, CheckCircle2 } from "lucide-react";

interface EvidenceItem {
  id: string;
  source: string;
  content_type: string;
  trust_score: number;
  content?: string;
}

interface EvidenceTableProps {
  evidence: EvidenceItem[];
}

export function EvidenceTable({ evidence }: EvidenceTableProps) {
  const [selectedItem, setSelectedItem] = useState<EvidenceItem | null>(null);

  const trustColor = (s: number) => {
    if (s >= 0.8) return "text-green-400 border-green-500/20 bg-green-500/5";
    if (s >= 0.5) return "text-yellow-400 border-yellow-500/20 bg-yellow-500/5";
    return "text-red-400 border-red-500/20 bg-red-500/5";
  };

  return (
    <>
      <Card className="glass relative overflow-hidden flex flex-col h-full">
        <CardHeader className="pb-3 border-b border-border/40">
          <div className="flex items-center gap-2">
            <Scale className="h-5 w-5 text-primary" />
            <CardTitle className="text-lg font-semibold tracking-tight">Evidence Artifacts</CardTitle>
          </div>
        </CardHeader>
        <CardContent className="flex-1 pt-4">
          <ScrollArea className="h-[280px] pr-2">
            <div className="space-y-2">
              {evidence.map((e) => (
                <div
                  key={e.id}
                  onClick={() => setSelectedItem(e)}
                  className="flex items-center justify-between p-3 rounded-lg border border-border/20 bg-secondary/25 hover:bg-secondary/50 hover:border-primary/20 transition-all duration-200 cursor-pointer group"
                >
                  <div className="space-y-0.5">
                    <p className="text-sm font-semibold text-foreground group-hover:text-primary transition-colors duration-200">
                      {e.source}
                    </p>
                    <p className="text-xs text-muted-foreground">{e.content_type}</p>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className={cn("font-mono text-xs px-2 py-0.5 border rounded-full font-bold", trustColor(e.trust_score).split(" ")[0], trustColor(e.trust_score).split(" ")[1])}>
                      {(e.trust_score * 100).toFixed(0)}%
                    </span>
                    <Eye className="h-4 w-4 text-muted-foreground group-hover:text-primary transition-colors opacity-0 group-hover:opacity-100" />
                  </div>
                </div>
              ))}
              {evidence.length === 0 && (
                <div className="text-center py-12 text-sm text-muted-foreground italic">
                  No evidence uploaded for this incident.
                </div>
              )}
            </div>
          </ScrollArea>
        </CardContent>
      </Card>

      {/* Log Inspector Dialog */}
      <Dialog open={!!selectedItem} onClose={() => setSelectedItem(null)}>
        {selectedItem && (
          <DialogContent className="glass-strong border border-border/60 max-w-xl shadow-2xl">
            <DialogHeader className="flex-row items-center justify-between pb-3 border-b border-border/40 space-y-0">
              <div className="flex items-center gap-2">
                <FileCode className="h-5 w-5 text-primary animate-pulse" />
                <DialogTitle className="text-base font-semibold text-foreground">
                  Evidence Inspector
                </DialogTitle>
              </div>
            </DialogHeader>

            <div className="space-y-4 mt-4">
              <div className="grid grid-cols-2 gap-4 text-xs">
                <div>
                  <span className="text-[10px] uppercase font-bold text-muted-foreground block mb-1">
                    Artifact Source
                  </span>
                  <span className="font-semibold text-foreground">{selectedItem.source}</span>
                </div>
                <div>
                  <span className="text-[10px] uppercase font-bold text-muted-foreground block mb-1">
                    Ingested Type
                  </span>
                  <span className="inline-flex items-center px-2 py-0.5 rounded-full bg-secondary text-foreground font-medium">
                    {selectedItem.content_type}
                  </span>
                </div>
              </div>

              <div>
                <span className="text-[10px] uppercase font-bold text-muted-foreground block mb-1">
                  Trust Score / Weight
                </span>
                <div className="flex items-center gap-2">
                  <CheckCircle2 className="h-4 w-4 text-green-400" />
                  <span className="text-sm font-mono font-bold text-foreground">
                    {(selectedItem.trust_score * 100).toFixed(0)}%
                  </span>
                  <span className="text-xs text-muted-foreground">(Extracted via Intelligence pipeline)</span>
                </div>
              </div>

              <div>
                <span className="text-[10px] uppercase font-bold text-muted-foreground block mb-1.5">
                  Raw Event payload
                </span>
                <pre className="p-4 rounded-lg bg-black/60 border border-border/60 text-xs font-mono text-cyan-400 overflow-x-auto whitespace-pre-wrap leading-relaxed max-h-60 overflow-y-auto">
                  {selectedItem.content || "No raw payload captured."}
                </pre>
              </div>

              <div className="flex justify-end pt-2 border-t border-border/40">
                <button
                  onClick={() => setSelectedItem(null)}
                  className="px-4 py-1.5 rounded-lg bg-secondary hover:bg-secondary/80 text-sm font-medium transition-colors"
                >
                  Close
                </button>
              </div>
            </div>
          </DialogContent>
        )}
      </Dialog>
    </>
  );
}
