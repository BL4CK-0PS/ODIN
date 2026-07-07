import { cn } from "@/lib/utils";
import { useState, type ReactNode } from "react";

interface Tab { label: string; value: string; content: ReactNode }

export function Tabs({ tabs, defaultTab }: { tabs: Tab[]; defaultTab?: string }) {
  const [active, setActive] = useState(defaultTab || tabs[0]?.value || "");
  return (
    <div>
      <div className="inline-flex h-10 items-center justify-center rounded-lg bg-secondary p-1 text-muted-foreground mb-2">
        {tabs.map((t) => (
          <button
            key={t.value}
            onClick={() => setActive(t.value)}
            className={cn(
              "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1.5 text-sm font-medium transition-all",
              active === t.value ? "bg-background text-foreground" : "hover:text-foreground"
            )}
          >
            {t.label}
          </button>
        ))}
      </div>
      {tabs.find((t) => t.value === active)?.content}
    </div>
  );
}
