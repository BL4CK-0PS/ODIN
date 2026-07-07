import { ScrollArea } from "@/components/ui/scroll-area";
import { cn } from "@/lib/utils";
import { Clock } from "lucide-react";

interface Event { id: string; source: string; type: string; collected_at: string }

export function TimelineView({ events }: { events: Event[] }) {
  return (
    <ScrollArea className="h-full pr-4">
      <div className="relative space-y-0">
        {events.map((event, i) => (
          <div key={event.id} className="flex gap-4 pb-8 relative">
            <div className="flex flex-col items-center">
              <div className={cn("w-3 h-3 rounded-full border-2 z-10", i === 0 ? "border-primary bg-primary/20" : "border-muted-foreground bg-card")} />
              {i < events.length - 1 && <div className="w-px flex-1 bg-border mt-1" />}
            </div>
            <div className="flex-1 pt-0.5">
              <div className="flex items-center gap-2 text-sm text-muted-foreground mb-1">
                <Clock className="h-3 w-3" />{new Date(event.collected_at).toLocaleString()}
              </div>
              <p className="text-sm font-medium">{event.source}</p>
              <p className="text-xs text-muted-foreground">{event.type}</p>
            </div>
          </div>
        ))}
      </div>
    </ScrollArea>
  );
}
