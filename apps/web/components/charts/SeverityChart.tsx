"use client";

import { useMemo, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ChartTooltip, ChartTooltipRow } from "@/components/ui/chart-tooltip";
import { cn } from "@/lib/utils";
import { useInvestigations } from "@/hooks/use-investigation";

const SEVERITY_COLORS: Record<string, string> = {
  Critical: "#DC2626",
  High: "#EA580C",
  Medium: "#EAB308",
  Low: "#16A34A",
  Informational: "#2563EB",
};

const SEVERITY_ORDER = ["Critical", "High", "Medium", "Low", "Informational"];

function polarToCartesian(cx: number, cy: number, r: number, angleDeg: number) {
  const rad = ((angleDeg - 90) * Math.PI) / 180;
  return { x: cx + r * Math.cos(rad), y: cy + r * Math.sin(rad) };
}

function describeArc(cx: number, cy: number, r: number, startAngle: number, endAngle: number) {
  const start = polarToCartesian(cx, cy, r, endAngle);
  const end = polarToCartesian(cx, cy, r, startAngle);
  const largeArc = endAngle - startAngle > 180 ? 1 : 0;
  return `M ${start.x} ${start.y} A ${r} ${r} 0 ${largeArc} 0 ${end.x} ${end.y}`;
}

function arcLength(cx: number, cy: number, r: number, startAngle: number, endAngle: number) {
  return ((endAngle - startAngle) / 360) * 2 * Math.PI * r;
}

interface DonutSlice {
  name: string;
  value: number;
  color: string;
  offset: number;
  pct: number;
}

export function SeverityChart() {
  const { data: incidents } = useInvestigations();
  const [hoveredIndex, setHoveredIndex] = useState<number | null>(null);

  const { slices, total } = useMemo(() => {
    if (!incidents || incidents.length === 0) {
      return { slices: [], total: 0 };
    }

    const counts: Record<string, number> = {};
    incidents.forEach((inc) => {
      counts[inc.severity] = (counts[inc.severity] || 0) + 1;
    });

    const total = incidents.length;
    let cumulativeOffset = 0;

    const slices: DonutSlice[] = SEVERITY_ORDER.filter((s) => counts[s]).map((name) => {
      const value = counts[name];
      const pct = Math.round((value / total) * 100);
      const slice = {
        name,
        value,
        color: SEVERITY_COLORS[name] || "#737373",
        offset: cumulativeOffset,
        pct,
      };
      cumulativeOffset += (value / total) * 360;
      return slice;
    });

    return { slices, total };
  }, [incidents]);

  const cx = 100, cy = 100, r = 70, strokeWidth = 20;

  return (
    <Card className="hover:shadow-medium transition-all duration-300">
      <CardHeader>
        <CardTitle className="text-sm font-medium text-muted-foreground">
          Severity Distribution
        </CardTitle>
      </CardHeader>
      <CardContent>
        {slices.length === 0 ? (
          <div className="flex aspect-video items-center justify-center text-xs text-muted-foreground">
            No investigations to display
          </div>
        ) : (
          <div className="flex items-center gap-8">
            <div className="relative shrink-0">
              <svg viewBox="0 0 200 200" className="w-44 h-44">
                {/* background track */}
                <circle
                  cx={cx}
                  cy={cy}
                  r={r}
                  fill="none"
                  stroke="hsl(0 0% 15%)"
                  strokeWidth={strokeWidth}
                  opacity={0.3}
                />

                {slices.map((slice, i) => {
                  const angle = (slice.value / total) * 360;
                  const gapDeg = slices.length > 1 ? 2.5 : 0;
                  const startAngle = slice.offset + gapDeg / 2;
                  const endAngle = slice.offset + angle - gapDeg / 2;
                  const isHovered = hoveredIndex === i;
                  const isAnyHovered = hoveredIndex !== null;
                  const pathD = describeArc(cx, cy, r, startAngle, endAngle);
                  const totalLen = arcLength(cx, cy, r, startAngle, endAngle);

                  if (endAngle - startAngle < 0.5) return null;

                  return (
                    <g key={slice.name}>
                      {/* invisible wider hover target */}
                      <path
                        d={pathD}
                        fill="none"
                        stroke="transparent"
                        strokeWidth={strokeWidth + 12}
                        strokeLinecap="round"
                        onMouseEnter={() => setHoveredIndex(i)}
                        onMouseLeave={() => setHoveredIndex(null)}
                        style={{ cursor: "default" }}
                      />
                      {/* visible arc */}
                      <ChartTooltip
                        side="top"
                        content={
                          <div className="space-y-1">
                            <ChartTooltipRow
                              color={slice.color}
                              label={slice.name}
                              value={`${slice.value} (${slice.pct}%)`}
                            />
                          </div>
                        }
                      >
                        <path
                          d={pathD}
                          fill="none"
                          stroke={slice.color}
                          strokeWidth={isHovered ? strokeWidth + 5 : strokeWidth}
                          strokeLinecap="round"
                          strokeDasharray={totalLen}
                          strokeDashoffset={0}
                          style={{
                            opacity: isAnyHovered && !isHovered ? 0.35 : 1,
                            transform: `scale(${isHovered ? 1.06 : 1})`,
                            transformOrigin: "center",
                            transition: "all 0.3s cubic-bezier(0.4, 0, 0.2, 1)",
                            animation: `donut-draw 0.8s ${i * 0.12}s cubic-bezier(0.4, 0, 0.2, 1) both`,
                          }}
                          onMouseEnter={() => setHoveredIndex(i)}
                          onMouseLeave={() => setHoveredIndex(null)}
                        />
                      </ChartTooltip>
                    </g>
                  );
                })}

                {/* center text */}
                <text
                  x={cx}
                  y={cy - 6}
                  textAnchor="middle"
                  className="fill-foreground"
                  fontSize={28}
                  fontWeight="700"
                  style={{
                    animation: "fade-in 0.5s 0.6s both",
                  }}
                >
                  {total}
                </text>
                <text
                  x={cx}
                  y={cy + 14}
                  textAnchor="middle"
                  className="fill-muted-foreground"
                  fontSize={10}
                  style={{
                    animation: "fade-in 0.5s 0.7s both",
                  }}
                >
                  incidents
                </text>
              </svg>
            </div>

            {/* legend */}
            <div className="flex flex-col gap-2.5 min-w-0">
              {slices.map((slice, i) => (
                <div
                  key={slice.name}
                  className="flex items-center gap-3 text-xs cursor-default group"
                  onMouseEnter={() => setHoveredIndex(i)}
                  onMouseLeave={() => setHoveredIndex(null)}
                  style={{
                    animation: `slide-in-left 0.3s ${0.3 + i * 0.06}s cubic-bezier(0.4, 0, 0.2, 1) both`,
                  }}
                >
                  <span
                    className="h-2.5 w-2.5 rounded-full shrink-0 transition-all duration-200"
                    style={{
                      backgroundColor: slice.color,
                      boxShadow: hoveredIndex === i ? `0 0 8px ${slice.color}` : "none",
                      transform: hoveredIndex === i ? "scale(1.35)" : "scale(1)",
                    }}
                  />
                  <span className={cn(
                    "text-muted-foreground truncate transition-colors",
                    hoveredIndex === i && "text-foreground"
                  )}>
                    {slice.name}
                  </span>
                  <span className="ml-auto flex items-baseline gap-1.5 tabular-nums">
                    <span className="font-mono font-medium text-foreground">{slice.value}</span>
                    <span className="font-mono text-[10px] text-muted-foreground">{slice.pct}%</span>
                  </span>
                </div>
              ))}
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
