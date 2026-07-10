"use client";

import { useMemo, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ChartTooltip, ChartTooltipRow } from "@/components/ui/chart-tooltip";
import type { Evidence } from "@/lib/types";

interface TrustScoreChartProps {
  evidence: Evidence[];
}

function getTrustColor(score: number): string {
  if (score >= 80) return "#16A34A";
  if (score >= 50) return "#CA8A04";
  return "#DC2626";
}

function getTrustLabel(score: number): string {
  if (score >= 80) return "High";
  if (score >= 50) return "Medium";
  return "Low";
}

export function TrustScoreChart({ evidence }: TrustScoreChartProps) {
  const [hoveredBar, setHoveredBar] = useState<string | null>(null);

  const data = useMemo(() => {
    return evidence.map((e) => {
      const score = Math.round(e.trust_score * 100);
      return {
        name: e.source,
        label: e.source.length > 16 ? e.source.slice(0, 14) + "..." : e.source,
        score,
        color: getTrustColor(score),
        trustLabel: getTrustLabel(score),
      };
    });
  }, [evidence]);

  const barHeight = 30;
  const gap = 10;
  const labelWidth = 110;
  const rightPad = 56;
  const gridLines = 4;
  const chartWidth = 400;
  const chartHeight = data.length * (barHeight + gap);

  return (
    <Card className="hover:shadow-medium transition-all duration-300">
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle className="text-sm font-medium text-muted-foreground">
            Evidence Trust Scores
          </CardTitle>
          <div className="flex items-center gap-3 text-[10px] text-muted-foreground">
            <span className="flex items-center gap-1">
              <span className="h-1.5 w-1.5 rounded-full bg-green-500" /> High
            </span>
            <span className="flex items-center gap-1">
              <span className="h-1.5 w-1.5 rounded-full bg-yellow-500" /> Medium
            </span>
            <span className="flex items-center gap-1">
              <span className="h-1.5 w-1.5 rounded-full bg-red-500" /> Low
            </span>
          </div>
        </div>
      </CardHeader>
      <CardContent>
        {data.length === 0 ? (
          <div className="flex items-center justify-center text-xs text-muted-foreground min-h-[120px]">
            No evidence to display
          </div>
        ) : (
          <svg viewBox={`0 0 ${chartWidth} ${chartHeight + 16}`} className="w-full" style={{ height: chartHeight + 16 }}>
            {/* vertical gridlines + x-axis labels */}
            {Array.from({ length: gridLines + 1 }, (_, i) => {
              const x = labelWidth + (i / gridLines) * (chartWidth - labelWidth - rightPad);
              const val = Math.round((i / gridLines) * 100);
              return (
                <g key={i}>
                  <line
                    x1={x}
                    y1={0}
                    x2={x}
                    y2={chartHeight}
                    stroke="hsl(0 0% 100%)"
                    strokeWidth={0.5}
                    opacity={0.07}
                  />
                  <text
                    x={x}
                    y={chartHeight + 14}
                    textAnchor="middle"
                    className="fill-muted-foreground"
                    fontSize={9}
                    opacity={0.5}
                  >
                    {val}%
                  </text>
                </g>
              );
            })}

            {data.map((item, i) => {
              const y = i * (barHeight + gap);
              const barWidth = (item.score / 100) * (chartWidth - labelWidth - rightPad);
              const isHovered = hoveredBar === item.name;

              return (
                <g
                  key={item.name}
                  onMouseEnter={() => setHoveredBar(item.name)}
                  onMouseLeave={() => setHoveredBar(null)}
                  style={{ cursor: "default" }}
                >
                  {isHovered && (
                    <rect
                      x={0}
                      y={y - 3}
                      width={chartWidth}
                      height={barHeight + 6}
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
                    fontSize={10}
                    style={{ transition: "fill 0.2s" }}
                    fill={isHovered ? "hsl(0 0% 90%)" : undefined}
                  >
                    {item.label}
                  </text>

                  <rect
                    x={labelWidth}
                    y={y + 3}
                    width={Math.max(barWidth, 4)}
                    height={barHeight - 6}
                    rx={4}
                    fill={item.color}
                    fillOpacity={isHovered ? 1 : 0.7}
                    className="chart-bar-h"
                    style={{
                      transition: "fill-opacity 0.2s",
                      animationDelay: `${i * 0.05}s`,
                    }}
                  />

                  {isHovered && (
                    <circle
                      cx={labelWidth + barWidth - 4}
                      cy={y + barHeight / 2}
                      r={3}
                      fill={item.color}
                      style={{ animation: "chart-fade-in 0.15s both" }}
                    />
                  )}

                  <text
                    x={labelWidth + barWidth + 10}
                    y={y + barHeight / 2 + 1}
                    dominantBaseline="middle"
                    className="fill-foreground"
                    fontSize={10}
                    fontWeight="600"
                    fontFamily="monospace"
                    style={{
                      transition: "fill 0.2s",
                      animation: "chart-fade-in 0.3s both",
                      animationDelay: `${0.2 + i * 0.05}s`,
                    }}
                    fill={isHovered ? item.color : undefined}
                  >
                    {item.score}%
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
