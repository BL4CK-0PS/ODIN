import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { cn } from "@/lib/utils";
import { Search } from "lucide-react";

export function SimilarityCard({ title, score, reasons, onClick }: { title: string; score: number; reasons: string[]; onClick?: () => void }) {
  return (
    <Card className={cn(onClick && "cursor-pointer transition-colors hover:border-primary/50")} onClick={onClick}>
      <CardHeader className="flex-row items-center justify-between space-y-0">
        <div className="flex items-center gap-2"><Search className="h-4 w-4 text-primary" /><CardTitle className="text-sm">{title}</CardTitle></div>
        <span className={cn("text-2xl font-mono font-bold", score >= 0.7 ? "text-green-400" : score >= 0.4 ? "text-yellow-400" : "text-red-400")}>{(score * 100).toFixed(0)}%</span>
      </CardHeader>
      <CardContent><div className="flex flex-wrap gap-1">{reasons.map((r, i) => (<Badge key={i} variant="outline" className="text-xs">{r}</Badge>))}</div></CardContent>
    </Card>
  );
}
