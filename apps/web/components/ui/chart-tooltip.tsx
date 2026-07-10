"use client";

import { useState, useRef, useCallback, type ReactNode } from "react";
import { cn } from "@/lib/utils";

interface ChartTooltipProps {
  children: ReactNode;
  content: ReactNode;
  side?: "top" | "bottom";
}

export function ChartTooltip({ children, content, side = "top" }: ChartTooltipProps) {
  const [visible, setVisible] = useState(false);
  const [pos, setPos] = useState({ x: 0, y: 0 });
  const ref = useRef<HTMLDivElement>(null);

  const onMove = useCallback((e: React.MouseEvent) => {
    if (!ref.current) return;
    const rect = ref.current.getBoundingClientRect();
    setPos({
      x: e.clientX - rect.left,
      y: e.clientY - rect.top,
    });
  }, []);

  return (
    <div
      ref={ref}
      className="relative inline-block"
      onMouseEnter={() => setVisible(true)}
      onMouseLeave={() => setVisible(false)}
      onMouseMove={onMove}
    >
      {children}
      {visible && (
        <div
          className={cn(
            "pointer-events-none absolute z-50 whitespace-nowrap rounded-lg border border-border/50 bg-background/95 px-2.5 py-1.5 text-xs shadow-xl backdrop-blur-sm",
            "animate-in fade-in-0 zoom-in-95 duration-150",
            side === "top" ? "bottom-full mb-2" : "top-full mt-2"
          )}
          style={{
            left: pos.x,
            transform: "translateX(-50%)",
          }}
        >
          {content}
        </div>
      )}
    </div>
  );
}

interface ChartTooltipRowProps {
  color?: string;
  label: string;
  value: string | number;
}

export function ChartTooltipRow({ color, label, value }: ChartTooltipRowProps) {
  return (
    <div className="flex items-center gap-2">
      {color && (
        <span
          className="h-2 w-2 shrink-0 rounded-full"
          style={{ backgroundColor: color }}
        />
      )}
      <span className="text-muted-foreground">{label}</span>
      <span className="ml-auto font-mono font-medium tabular-nums text-foreground">
        {value}
      </span>
    </div>
  );
}
