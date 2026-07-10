"use client";

import { ScrollArea } from "@/components/ui/scroll-area";
import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";
import { Clock, FileText, Globe, Key, Shield, AlertTriangle } from "lucide-react";

interface TimelineEvent {
  id: string;
  source: string;
  type: string;
  collected_at: string;
}

interface TimelineViewProps {
  events: TimelineEvent[];
}

export function TimelineView({ events }: TimelineViewProps) {
  const getEventIcon = (type: string) => {
    const t = type.toLowerCase();
    if (t.includes("network") || t.includes("traffic")) return Globe;
    if (t.includes("auth") || t.includes("cred") || t.includes("user")) return Key;
    if (t.includes("intel") || t.includes("threat")) return AlertTriangle;
    if (t.includes("memory") || t.includes("dump")) return Shield;
    return FileText;
  };

  const getEventColor = (type: string) => {
    const t = type.toLowerCase();
    if (t.includes("network") || t.includes("traffic")) return "border-emerald-500/30 text-emerald-400 bg-emerald-500/5";
    if (t.includes("auth") || t.includes("cred") || t.includes("user")) return "border-rose-500/30 text-rose-400 bg-rose-500/5";
    if (t.includes("intel") || t.includes("threat")) return "border-amber-500/30 text-amber-400 bg-amber-500/5";
    if (t.includes("memory") || t.includes("dump")) return "border-purple-500/30 text-purple-400 bg-purple-500/5";
    return "border-blue-500/30 text-blue-400 bg-blue-500/5";
  };

  // Sort events chronologically (newest last or first? typically timeline is newest first or chronological. Let's sort oldest first to show progression)
  const sortedEvents = [...events].sort(
    (a, b) => new Date(a.collected_at).getTime() - new Date(b.collected_at).getTime()
  );

  return (
    <ScrollArea className="h-full pr-4 select-none">
      <div className="relative pl-6 space-y-0 border-l border-border/60 ml-4 py-2">
        {sortedEvents.map((event, i) => {
          const Icon = getEventIcon(event.type);
          const colorClass = getEventColor(event.type);
          const isLatest = i === sortedEvents.length - 1;

          return (
            <div key={event.id} className="relative pb-8 group last:pb-2">
              {/* Timeline Connector Dot */}
              <div
                className={cn(
                  "absolute -left-[31px] top-1.5 w-4 h-4 rounded-full border bg-background flex items-center justify-center transition-all duration-300 group-hover:scale-110",
                  isLatest
                    ? "border-accent-foreground pulse-dot"
                    : "border-muted-foreground/50"
                )}
                style={isLatest ? { backgroundColor: "hsl(var(--accent-foreground) / 0.15)" } : undefined}
              >
                <div
                  className={cn(
                    "w-1.5 h-1.5 rounded-full",
                    isLatest ? "bg-accent-foreground" : "bg-muted-foreground/60"
                  )}
                />
              </div>

              {/* Event Card Content */}
              <div className="p-4 rounded-xl border border-border/40 bg-card/40 backdrop-blur-sm transition-all duration-300 group-hover:border-accent-foreground/20 group-hover:bg-card/70 group-hover:-translate-y-0.5">
                <div className="flex flex-wrap items-center justify-between gap-2 mb-2">
                  <span className="flex items-center gap-1.5 text-xs text-muted-foreground">
                    <Clock className="h-3.5 w-3.5" />
                    {new Date(event.collected_at).toLocaleString()}
                  </span>
                  <Badge variant="outline" className={cn("text-[10px] px-2 py-0.5 font-mono", colorClass)}>
                    <Icon className="h-3 w-3 mr-1" />
                    {event.type}
                  </Badge>
                </div>

                <p className="text-sm font-semibold text-foreground tracking-tight mb-1">
                  {event.source}
                </p>
                <p className="text-xs text-muted-foreground leading-relaxed">
                  Artifact event recorded in investigation node timeline.
                </p>
              </div>
            </div>
          );
        })}

        {sortedEvents.length === 0 && (
          <div className="text-center py-12 text-sm text-muted-foreground">
            No events in this investigation timeline.
          </div>
        )}
      </div>
    </ScrollArea>
  );
}
