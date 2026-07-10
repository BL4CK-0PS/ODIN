"use client";

import { useMemo, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ChartTooltip, ChartTooltipRow } from "@/components/ui/chart-tooltip";
import { useGlobalGraph } from "@/hooks/use-graph";
import { useGraphStore } from "@/stores/graph";

const TYPE_COLORS: Record<string, string> = {
  incident: "#DC2626",
  evidence: "#0891B2",
  ipaddress: "#16A34A",
  domain: "#CA8A04",
  process: "#7C3AED",
  file: "#EAB308",
  user: "#E11D48",
  hash: "#7C3AED",
  hostname: "#2563EB",
  networkconnection: "#10B981",
  artifact: "#737373",
};

const TYPE_LABELS: Record<string, string> = {
  incident: "Incident",
  evidence: "Evidence",
  ipaddress: "IP Address",
  domain: "Domain",
  process: "Process",
  file: "File",
  user: "User",
  hash: "Hash",
  hostname: "Hostname",
  networkconnection: "Network",
  artifact: "Artifact",
};

export function EntityTypeChart() {
  const { isLoading } = useGlobalGraph();
  const nodes = useGraphStore((s) => s.nodes);
  const [hoveredBar, setHoveredBar] = useState<string | null>(null);

  const { data, maxCount } = useMemo(() => {
    if (!nodes || nodes.length === 0) return { data: [], maxCount: 0 };

    const counts: Record<string, number> = {};
    nodes.forEach((n) => {
      const t = n.type.toLowerCase();
      counts[t] = (counts[t] || 0) + 1;
    });

    const data = Object.entries(counts)
      .map(([name, count]) => ({
        name,
        label: TYPE_LABELS[name] || name.charAt(0).toUpperCase() + name.slice(1),
        count,
        color: TYPE_COLORS[name] || "#737373",
      }))
      .sort((a, b) => b.count - a.count)
      .slice(0, 8);

    const maxCount = Math.max(...data.map((d) => d.count), 1);
    return { data, maxCount };
  }, [nodes]);

  const barHeight = 28;
  const gap = 8;
  const labelWidth = 90;
  const rightPad = 50;
  const gridLines = 4;
  const chartWidth = 400;
  const chartHeight = data.length * (barHeight + gap);

  return (
    <Card className="hover:shadow-medium transition-all duration-300">
      <CardHeader>
        <CardTitle className="text-sm font-medium text-muted-foreground">
          Entity Types
        </CardTitle>
      </CardHeader>
      <CardContent>
        {isLoading ? (
          <div className="flex aspect-video items-center justify-center text-xs text-muted-foreground animate-pulse">
            Loading graph data...
          </div>
        ) : data.length === 0 ? (
          <div className="flex aspect-video items-center justify-center text-xs text-muted-foreground">
            No entities to display
          </div>
        ) : (
          <svg viewBox={`0 0 ${chartWidth} ${chartHeight + 4}`} className="w-full" style={{ height: chartHeight + 4 }}>
            {/* vertical gridlines */}
            {Array.from({ length: gridLines + 1 }, (_, i) => {
              const x = labelWidth + (i / gridLines) * (chartWidth - labelWidth - rightPad);
              const val = Math.round((i / gridLines) * maxCount);
              return (
                <g key={i}>
                  <line
                    x1={x}
                    y1={0}
                    x2={x}
                    y2={chartHeight}
                    stroke="hsl(0 0% 100%)"
                    strokeWidth={0.5}
                    opacity={0.08}
                  />
                  <text
                    x={x}
                    y={chartHeight + 2}
                    textAnchor="middle"
                    className="fill-muted-foreground"
                    fontSize={9}
                    opacity={0.6}
                  >
                    {val}
                  </text>
                </g>
              );
            })}

            {data.map((item, i) => {
              const y = i * (barHeight + gap);
              const barWidth = (item.count / maxCount) * (chartWidth - labelWidth - rightPad);
              const isHovered = hoveredBar === item.name;

              return (
                <g
                  key={item.name}
                  onMouseEnter={() => setHoveredBar(item.name)}
                  onMouseLeave={() => setHoveredBar(null)}
                  style={{ cursor: "default" }}
                >
                  {/* row highlight on hover */}
                  {isHovered && (
                    <rect
                      x={0}
                      y={y - 2}
                      width={chartWidth}
                      height={barHeight + 4}
                      rx={6}
                      fill="hsl(0 0% 100%)"
                      opacity={0.03}
                    />
                  )}

                  <text
                    x={labelWidth - 10}
                    y={y + barHeight / 2 + 1}
                    textAnchor="end"
                    dominantBaseline="middle"
                    className="fill-muted-foreground"
                    fontSize={11}
                    style={{ transition: "fill 0.2s" }}
                    fill={isHovered ? "hsl(0 0% 90%)" : undefined}
                  >
                    {item.label}
                  </text>

                  <rect
                    x={labelWidth}
                    y={y + 2}
                    width={Math.max(barWidth, 4)}
                    height={barHeight - 4}
                    rx={4}
                    fill={item.color}
                    fillOpacity={isHovered ? 1 : 0.7}
                    style={{
                      transition: "all 0.25s cubic-bezier(0.4, 0, 0.2, 1)",
                      animation: `bar-grow 0.5s ${i * 0.06}s cubic-bezier(0.4, 0, 0.2, 1) both`,
                    }}
                  />

                  {/* glow dot on hover */}
                  {isHovered && (
                    <circle
                      cx={labelWidth + barWidth - 4}
                      cy={y + barHeight / 2}
                      r={3}
                      fill={item.color}
                      style={{ animation: "fade-in 0.15s both" }}
                    />
                  )}

                  <text
                    x={labelWidth + barWidth + 10}
                    y={y + barHeight / 2 + 1}
                    dominantBaseline="middle"
                    className="fill-foreground"
                    fontSize={11}
                    fontWeight="600"
                    fontFamily="monospace"
                    style={{
                      transition: "fill 0.2s",
                      animation: `fade-in 0.3s ${0.3 + i * 0.06}s both`,
                    }}
                    fill={isHovered ? item.color : undefined}
                  >
                    {item.count}
                  </text>
                </g>
              );
            })}
          </svg>
        )}
      </CardContent>
    </Card>
  );
}
