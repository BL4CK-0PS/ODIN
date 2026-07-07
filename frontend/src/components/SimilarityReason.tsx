import { Card, CardContent } from "@/components/ui/card";
import { CheckCircle2, XCircle } from "lucide-react";

interface Reason { label: string; matched: boolean }

export function SimilarityReason({ reasons }: { reasons: Reason[] }) {
  return (
    <Card><CardContent className="p-4 space-y-2">{reasons.map((r, i) => (
      <div key={i} className="flex items-center gap-2 text-sm">
        {r.matched ? <CheckCircle2 className="h-4 w-4 text-green-400 shrink-0" /> : <XCircle className="h-4 w-4 text-red-400 shrink-0" />}
        <span className={r.matched ? "text-foreground" : "text-muted-foreground"}>{r.label}</span>
      </div>
    ))}</CardContent></Card>
  );
}
