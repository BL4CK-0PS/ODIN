"use client";

import { useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { BookOpen, CheckCircle2, Circle } from "lucide-react";

interface PlaybookCardProps {
  name: string;
  steps: string[];
}

export function PlaybookCard({ name, steps }: PlaybookCardProps) {
  const [completedSteps, setCompletedSteps] = useState<Record<number, boolean>>({});

  const toggleStep = (index: number) => {
    setCompletedSteps((prev) => ({
      ...prev,
      [index]: !prev[index],
    }));
  };

  const totalSteps = steps.length;
  const completedCount = Object.values(completedSteps).filter(Boolean).length;
  const progressPercent = totalSteps > 0 ? Math.round((completedCount / totalSteps) * 100) : 0;

  return (
    <Card className="glass relative overflow-hidden flex flex-col h-full">
      <CardHeader className="pb-3 border-b border-border/40">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <BookOpen className="h-5 w-5 text-primary" />
            <CardTitle className="text-lg font-semibold tracking-tight">{name}</CardTitle>
          </div>
          <span className="text-xs font-mono font-bold text-primary bg-primary/10 px-2 py-0.5 rounded-full">
            {completedCount}/{totalSteps}
          </span>
        </div>
        {/* Dynamic Progress Bar */}
        <div className="w-full h-1 bg-border/40 rounded-full mt-3 overflow-hidden">
          <div
            className="h-full bg-primary transition-all duration-500 ease-out"
            style={{ width: `${progressPercent}%` }}
          />
        </div>
      </CardHeader>
      
      <CardContent className="flex-1 pt-4">
        <ul className="space-y-3">
          {steps.map((step, i) => {
            const isCompleted = !!completedSteps[i];

            return (
              <li
                key={i}
                onClick={() => toggleStep(i)}
                className="flex items-start gap-3 p-2.5 rounded-lg border border-transparent cursor-pointer select-none transition-all duration-200 hover:bg-secondary/40 hover:border-border/30"
              >
                <div className="mt-0.5 shrink-0 transition-transform duration-200 active:scale-90">
                  {isCompleted ? (
                    <CheckCircle2 className="h-4.5 w-4.5 text-green-400 fill-green-500/10" />
                  ) : (
                    <Circle className="h-4.5 w-4.5 text-muted-foreground/60 hover:text-primary" />
                  )}
                </div>
                <span
                  className={`text-sm leading-relaxed transition-all duration-300 ${
                    isCompleted ? "line-through text-muted-foreground/50 font-normal" : "text-foreground font-medium"
                  }`}
                >
                  {step}
                </span>
              </li>
            );
          })}
        </ul>
      </CardContent>
    </Card>
  );
}
