"use client";

import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { cn } from "@/lib/utils";
import { Search, Flame } from "lucide-react";

interface SimilarityCardProps {
  title: string;
  score: number;
  reasons: string[];
  onClick?: () => void;
}

export function SimilarityCard({ title, score, reasons, onClick }: SimilarityCardProps) {
  const isHigh = score >= 0.7;
  const scoreColor = isHigh ? "stroke-green-400 text-green-400" : score >= 0.4 ? "stroke-yellow-400 text-yellow-400" : "stroke-red-400 text-red-400";

  const radius = 18;
  const circumference = 2 * Math.PI * radius;
  const strokeDashoffset = circumference - score * circumference;

  return (
    <Card
      className={cn(
        "glass border border-border/40 hover:border-accent-foreground/20 hover:shadow-warm transition-all duration-300 group",
        onClick && "cursor-pointer active:scale-[0.99]"
      )}
      onClick={onClick}
    >
      <CardHeader className="flex-row items-center justify-between pb-3 space-y-0">
        <div className="flex items-center gap-3">
          <div className="flex items-center justify-center w-10 h-10 rounded-xl bg-accent/30 group-hover:bg-accent/50 transition-colors duration-300">
            <Search className="h-4 w-4 text-accent-foreground" />
          </div>
          <div className="space-y-0.5">
            <CardTitle className="text-sm font-semibold tracking-tight text-foreground">
              {title}
            </CardTitle>
            {isHigh && (
              <span className="inline-flex items-center gap-1 text-[10px] text-red-400/90 font-medium font-mono">
                <Flame className="h-3 w-3" />
                Critical Similarity
              </span>
            )}
          </div>
        </div>

        <div className="relative flex items-center justify-center w-12 h-12">
          <svg className="w-12 h-12 -rotate-90">
            <circle
              cx="24"
              cy="24"
              r={radius}
              className="score-ring-track"
              strokeWidth="2.5"
            />
            <circle
              cx="24"
              cy="24"
              r={radius}
              className={cn("score-ring-fill transition-all duration-1000", scoreColor)}
              strokeWidth="3"
              strokeDasharray={circumference}
              strokeDashoffset={strokeDashoffset}
            />
          </svg>
          <span className={cn("absolute text-xs font-mono font-extrabold", scoreColor.split(" ")[1])}>
            {(score * 100).toFixed(0)}
          </span>
        </div>
      </CardHeader>

      <CardContent>
        <div className="flex flex-wrap gap-1.5 pt-1">
          {reasons.map((r, i) => (
            <Badge
              key={i}
              variant="outline"
              className="text-[10px] font-mono py-0.5 px-2 bg-secondary/20 border-border/60 hover:bg-accent/30 hover:border-accent-foreground/20 hover:text-accent-foreground transition-all duration-300"
            >
              {r}
            </Badge>
          ))}
          {reasons.length === 0 && (
            <span className="text-xs text-muted-foreground italic">No similarity match reason factors recorded.</span>
          )}
        </div>
      </CardContent>
    </Card>
  );
}
