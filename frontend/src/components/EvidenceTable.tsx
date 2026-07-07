import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { cn } from "@/lib/utils";
import { Scale } from "lucide-react";

interface EvidenceItem { id: string; source: string; content_type: string; trust_score: number }

export function EvidenceTable({ evidence }: { evidence: EvidenceItem[] }) {
  return (
    <Card>
      <CardHeader><div className="flex items-center gap-2"><Scale className="h-5 w-5 text-primary" /><CardTitle className="text-lg">Evidence</CardTitle></div></CardHeader>
      <CardContent>
        <ScrollArea className="max-h-80">
          <div className="space-y-2">{evidence.map((e) => (
            <div key={e.id} className="flex items-center justify-between p-3 rounded-lg bg-secondary/50">
              <div><p className="text-sm font-medium">{e.source}</p><p className="text-xs text-muted-foreground">{e.content_type}</p></div>
              <span className={cn("font-mono text-sm font-medium", e.trust_score >= 0.8 ? "text-green-400" : e.trust_score >= 0.5 ? "text-yellow-400" : "text-red-400")}>{(e.trust_score * 100).toFixed(0)}</span>
            </div>
          ))}</div>
        </ScrollArea>
      </CardContent>
    </Card>
  );
}
