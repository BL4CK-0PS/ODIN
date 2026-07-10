"use client";

import { useMemo, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ChartTooltip, ChartTooltipRow } from "@/components/ui/chart-tooltip";
import type { MemoryObject } from "@/lib/types";

interface ConfidenceHistogramProps {
  memories: MemoryObject[];
}

const BUCKETS = [
  { label: "0–20", min: 0, max: 20 },
  { label: "20–40", min: 20, max: 40 },
  { label: "40–60", min: 40, max: 60 },
  { label: "60–80", min: 60, max: 80 },
  { label: "80–100", min: 80, max: 100 },
];

function getBarColor(count: number, isHovered: boolean): string {
  if (count === 0) return "rgba(255,255,255,0.06)";
  if (isHovered) return "#E56A4A";
  return "rgba(229,106,74,0.8)";
}

function getBarGradientId(index: number) {
  return `hist-grad-${index}`;
}

export function ConfidenceHistogram({ memories }: ConfidenceHistogramProps) {
  const [hoveredBar, setHoveredBar] = useState<number | null>(null);

  const { data, maxCount } = useMemo(() => {
    const data = BUCKETS.map((b, i) => ({
      label: b.label,
      count: memories.filter(
        (m) => (m.confidence ?? 0) * 100 >= b.min && (m.confidence ?? 0) * 100 < b.max
      ).length,
      index: i,
    }));
    const maxCount = Math.max(...data.map((d) => d.count), 1);
    return { data, maxCount };
  }, [memories]);

  const hasData = data.some((d) => d.count > 0);

  const chartWidth = 360;
  const chartHeight = 200;
  const barWidth = 44;
  const gap = (chartWidth - data.length * barWidth) / (data.length + 1);
  const bottomPad = 30;
  const topPad = 20;
  const drawHeight = chartHeight - bottomPad - topPad;
  const gridLines = 4;

  return (
    <Card className="hover:shadow-medium transition-all duration-300">
      <CardHeader>
        <CardTitle className="text-sm font-medium text-muted-foreground">
          Confidence Distribution
        </CardTitle>
      </CardHeader>
      <CardContent>
        {!hasData ? (
          <div className="flex items-center justify-center text-xs text-muted-foreground min-h-[180px]">
            No memories to display
          </div>
        ) : (
          <svg viewBox={`0 0 ${chartWidth} ${chartHeight}`} className="w-full" style={{ height: chartHeight }}>
            <defs>
              {data.map((item) => (
                <linearGradient key={item.label} id={getBarGradientId(item.index)} x1="0" y1="0" x2="0" y2="1">
                  <stop offset="0%" stopColor="#E56A4A" stopOpacity={0.9} />
                  <stop offset="100%" stopColor="#E56A4A" stopOpacity={0.4} />
                </linearGradient>
              ))}
            </defs>

            {/* horizontal gridlines */}
            {Array.from({ length: gridLines + 1 }, (_, i) => {
              const y = topPad + (i / gridLines) * drawHeight;
              const val = Math.round(((gridLines - i) / gridLines) * maxCount);
              return (
                <g key={i}>
                  <line
                    x1={0}
                    y1={y}
                    x2={chartWidth}
                    y2={y}
                    stroke="hsl(0 0% 100%)"
                    strokeWidth={0.5}
                    opacity={0.06}
                  />
                  {i < gridLines && (
                    <text
                      x={4}
                      y={y - 4}
                      className="fill-muted-foreground"
                      fontSize={9}
                      opacity={0.5}
                    >
                      {val}
                    </text>
                  )}
                </g>
              );
            })}

            {/* baseline */}
            <line
              x1={0}
              y1={chartHeight - bottomPad}
              x2={chartWidth}
              y2={chartHeight - bottomPad}
              stroke="hsl(0 0% 100%)"
              strokeWidth={0.5}
              opacity={0.15}
            />

            {data.map((item, i) => {
              const x = gap + i * (barWidth + gap);
              const barH = (item.count / maxCount) * drawHeight;
              const y = chartHeight - bottomPad - barH;
              const isHovered = hoveredBar === i;

              return (
                <g
                  key={item.label}
                  onMouseEnter={() => setHoveredBar(i)}
                  onMouseLeave={() => setHoveredBar(null)}
                  style={{ cursor: "default" }}
                >
                  <rect
                    x={x}
                    y={y}
                    width={barWidth}
                    height={Math.max(barH, 2)}
                    rx={4}
                    fill={item.count > 0 ? `url(#${getBarGradientId(i)})` : getBarColor(item.count, isHovered)}
                    fillOpacity={isHovered ? 1 : item.count === 0 ? 0.3 : 0.85}
                    className="chart-bar-v"
                    style={{
                      transition: "fill-opacity 0.2s",
                      animationDelay: `${i * 0.08}s`,
                    }}
                  />

                  {/* glow on hover */}
                  {isHovered && item.count > 0 && (
                    <rect
                      x={x - 2}
                      y={y - 2}
                      width={barWidth + 4}
                      height={Math.max(barH, 2) + 4}
                      rx={6}
                      fill="none"
                      stroke="#E56A4A"
                      strokeWidth={1}
                      opacity={0.3}
                      style={{ animation: "chart-fade-in 0.15s both" }}
                    />
                  )}

                  {/* count label above bar */}
                  {item.count > 0 && (
                    <text
                      x={x + barWidth / 2}
                      y={y - 8}
                      textAnchor="middle"
                      className="fill-foreground"
                      fontSize={11}
                      fontWeight="600"
                      fontFamily="monospace"
                      style={{
                        animation: "chart-fade-in 0.3s both",
                        animationDelay: `${0.3 + i * 0.08}s`,
                      }}
                    >
                      {item.count}
                    </text>
                  )}

                  {/* x-axis label */}
                  <text
                    x={x + barWidth / 2}
                    y={chartHeight - bottomPad + 16}
                    textAnchor="middle"
                    className="fill-muted-foreground"
                    fontSize={10}
                  >
                    {item.label}%
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
