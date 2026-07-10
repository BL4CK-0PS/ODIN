"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { GitCompare } from "lucide-react";

interface DiffCardProps {
  title: string;
  current: string;
  previous: string;
  type: "added" | "removed" | "changed";
}

export function DiffCard({ title, current, previous, type }: DiffCardProps) {
  const colors = {
    added: "border-green-500/30 bg-green-500/5",
    removed: "border-red-500/30 bg-red-500/5",
    changed: "border-yellow-500/30 bg-yellow-500/5",
  };

  return (
    <Card className={colors[type]}>
      <CardHeader>
        <div className="flex items-center gap-2">
          <GitCompare className="h-4 w-4 text-accent-foreground" />
          <CardTitle className="text-sm">{title}</CardTitle>
        </div>
      </CardHeader>
      <CardContent className="space-y-2 font-mono text-xs">
        <div className="text-red-400 line-through">- {previous}</div>
        <div className="text-green-400">+ {current}</div>
      </CardContent>
    </Card>
  );
}
