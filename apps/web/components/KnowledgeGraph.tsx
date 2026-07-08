"use client";

import React, { useState, useEffect, useRef, useMemo } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Button } from "@/components/ui/button";
import { Brain, ZoomIn, ZoomOut, RotateCcw, Shield, FileText, Globe, Terminal, User } from "lucide-react";

interface GraphNode {
  id: string;
  type: string;
  label: string;
}

interface GraphEdge {
  source: string;
  target: string;
  type?: string;
  label?: string;
}

interface KnowledgeGraphProps {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

interface Position {
  x: number;
  y: number;
}

export function KnowledgeGraph({ nodes, edges }: KnowledgeGraphProps) {
  const containerRef = useRef<SVGSVGElement | null>(null);
  const [positions, setPositions] = useState<Record<string, Position>>({});
  const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null);
  const [hoveredNode, setHoveredNode] = useState<string | null>(null);
  const [draggedNode, setDraggedNode] = useState<string | null>(null);
  const [pan, setPan] = useState({ x: 300, y: 200 });
  const [zoom, setZoom] = useState(1);
  const [isPanning, setIsPanning] = useState(false);
  const panStart = useRef({ x: 0, y: 0 });

  // Map node types to Lucide Icons
  const getNodeIcon = (type: string) => {
    switch (type.toLowerCase()) {
      case "incident":
        return Brain;
      case "evidence":
        return FileText;
      case "ipaddress":
      case "domain":
        return Globe;
      case "process":
        return Terminal;
      case "file":
        return FileText;
      case "user":
        return User;
      default:
        return Shield;
    }
  };

  // Node styles configuration
  const getNodeColorClass = (type: string) => {
    switch (type.toLowerCase()) {
      case "incident":
        return "text-blue-400 stroke-blue-500 fill-blue-500/20";
      case "evidence":
        return "text-cyan-400 stroke-cyan-500 fill-cyan-500/20";
      case "ipaddress":
      case "domain":
        return "text-emerald-400 stroke-emerald-500 fill-emerald-500/20";
      case "process":
        return "text-purple-400 stroke-purple-500 fill-purple-500/20";
      case "file":
        return "text-amber-400 stroke-amber-500 fill-amber-500/20";
      case "user":
        return "text-rose-400 stroke-rose-500 fill-rose-500/20";
      default:
        return "text-muted-foreground stroke-border fill-muted/20";
    }
  };

  // Calculate layout using simple force-directed simulation
  useEffect(() => {
    if (nodes.length === 0) return;

    // Initialize positions randomly in a circle
    let tempPos: Record<string, Position> = {};
    nodes.forEach((node, i) => {
      const angle = (i / nodes.length) * 2 * Math.PI;
      const radius = 100 + Math.random() * 30;
      tempPos[node.id] = {
        x: Math.cos(angle) * radius,
        y: Math.sin(angle) * radius,
      };
    });

    const k = 100; // Ideal distance
    const gravity = 0.05;
    const repulsion = 10000;
    const attraction = 0.05;

    // Run simulation steps
    for (let step = 0; step < 120; step++) {
      let forces: Record<string, Position> = {};
      nodes.forEach((n) => {
        forces[n.id] = { x: 0, y: 0 };
      });

      // Repulsion force between all nodes
      for (let i = 0; i < nodes.length; i++) {
        for (let j = i + 1; j < nodes.length; j++) {
          const n1 = nodes[i];
          const n2 = nodes[j];
          const dx = tempPos[n2.id].x - tempPos[n1.id].x;
          const dy = tempPos[n2.id].y - tempPos[n1.id].y;
          const dist = Math.sqrt(dx * dx + dy * dy) || 1;
          
          if (dist < 300) {
            const force = repulsion / (dist * dist);
            const fx = (dx / dist) * force;
            const fy = (dy / dist) * force;
            
            forces[n1.id].x -= fx;
            forces[n1.id].y -= fy;
            forces[n2.id].x += fx;
            forces[n2.id].y += fy;
          }
        }
      }

      // Attraction force along edges
      edges.forEach((edge) => {
        const source = tempPos[edge.source];
        const target = tempPos[edge.target];
        if (!source || !target) return;

        const dx = target.x - source.x;
        const dy = target.y - source.y;
        const dist = Math.sqrt(dx * dx + dy * dy) || 1;
        const force = attraction * (dist - k);
        const fx = (dx / dist) * force;
        const fy = (dy / dist) * force;

        forces[edge.source].x += fx;
        forces[edge.source].y += fy;
        forces[edge.target].x -= fx;
        forces[edge.target].y -= fy;
      });

      // Center gravity
      nodes.forEach((node) => {
        forces[node.id].x -= tempPos[node.id].x * gravity;
        forces[node.id].y -= tempPos[node.id].y * gravity;
      });

      // Update positions
      nodes.forEach((node) => {
        const f = forces[node.id];
        const limit = 12;
        const fx = Math.max(-limit, Math.min(limit, f.x));
        const fy = Math.max(-limit, Math.min(limit, f.y));
        tempPos[node.id].x += fx;
        tempPos[node.id].y += fy;
      });
    }

    setPositions(tempPos);
  }, [nodes, edges]);

  // Mouse drag handling
  const handleMouseDown = (nodeId: string, e: React.MouseEvent) => {
    e.stopPropagation();
    setDraggedNode(nodeId);
    setSelectedNode(nodes.find((n) => n.id === nodeId) || null);
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    if (draggedNode && positions[draggedNode]) {
      const rect = containerRef.current?.getBoundingClientRect();
      if (!rect) return;
      const x = (e.clientX - rect.left - pan.x) / zoom;
      const y = (e.clientY - rect.top - pan.y) / zoom;
      setPositions((prev) => ({
        ...prev,
        [draggedNode]: { x, y },
      }));
    } else if (isPanning) {
      const dx = e.clientX - panStart.current.x;
      const dy = e.clientY - panStart.current.y;
      setPan((prev) => ({ x: prev.x + dx, y: prev.y + dy }));
      panStart.current = { x: e.clientX, y: e.clientY };
    }
  };

  const handleMouseUp = () => {
    setDraggedNode(null);
    setIsPanning(false);
  };

  const handleBgMouseDown = (e: React.MouseEvent) => {
    setIsPanning(true);
    panStart.current = { x: e.clientX, y: e.clientY };
  };

  const resetViewport = () => {
    setPan({ x: 300, y: 200 });
    setZoom(1);
  };

  // Connected nodes helper for highlighting paths
  const connectedNodeIds = useMemo(() => {
    if (!hoveredNode) return new Set<string>();
    const neighbors = new Set<string>([hoveredNode]);
    edges.forEach((edge) => {
      if (edge.source === hoveredNode) neighbors.add(edge.target);
      if (edge.target === hoveredNode) neighbors.add(edge.source);
    });
    return neighbors;
  }, [hoveredNode, edges]);

  return (
    <div className="grid grid-cols-4 gap-4 h-[450px]">
      {/* Dynamic Inspector Panel */}
      <Card className="col-span-1 glass flex flex-col h-full overflow-hidden">
        <CardHeader className="pb-3 border-b border-border/40">
          <div className="flex items-center gap-2">
            <Brain className="h-5 w-5 text-primary animate-pulse" />
            <CardTitle className="text-base font-semibold">Node Inspector</CardTitle>
          </div>
        </CardHeader>
        <CardContent className="flex-1 p-3 overflow-hidden">
          {selectedNode ? (
            <ScrollArea className="h-full pr-1">
              <div className="space-y-4">
                <div>
                  <span className="text-[10px] uppercase font-bold tracking-wider text-muted-foreground block mb-1">
                    Entity Type
                  </span>
                  <span className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium bg-primary/10 border border-primary/20 text-primary">
                    {React.createElement(getNodeIcon(selectedNode.type), { className: "h-3.5 w-3.5" })}
                    {selectedNode.type}
                  </span>
                </div>

                <div>
                  <span className="text-[10px] uppercase font-bold tracking-wider text-muted-foreground block mb-1">
                    Label
                  </span>
                  <p className="text-sm font-semibold break-words leading-relaxed text-foreground">
                    {selectedNode.label}
                  </p>
                </div>

                <div>
                  <span className="text-[10px] uppercase font-bold tracking-wider text-muted-foreground block mb-2">
                    Direct Connections
                  </span>
                  <div className="space-y-2">
                    {edges
                      .filter((e) => e.source === selectedNode.id || e.target === selectedNode.id)
                      .map((edge, i) => {
                        const targetId = edge.source === selectedNode.id ? edge.target : edge.source;
                        const targetNode = nodes.find((n) => n.id === targetId);
                        const rel = edge.label || edge.type || "connected_to";
                        return (
                          <div
                            key={i}
                            className="text-xs p-2 rounded bg-muted/30 border border-border/20 flex flex-col gap-1 cursor-pointer hover:bg-muted/50"
                            onClick={() => setSelectedNode(targetNode || null)}
                          >
                            <span className="text-[10px] font-mono text-primary/80">{rel}</span>
                            <span className="font-medium text-foreground">{targetNode?.label || "Unknown"}</span>
                          </div>
                        );
                      })}
                  </div>
                </div>
              </div>
            </ScrollArea>
          ) : (
            <div className="h-full flex flex-col items-center justify-center text-center p-4">
              <Shield className="h-8 w-8 text-muted-foreground/30 mb-2" />
              <p className="text-xs text-muted-foreground leading-relaxed">
                Click a node in the graph to inspect entity attributes and direct relationships.
              </p>
            </div>
          )}
        </CardContent>
      </Card>

      {/* SVG Canvas Area */}
      <Card className="col-span-3 glass relative h-full overflow-hidden flex flex-col">
        {/* Navigation Viewport Controls */}
        <div className="absolute top-3 right-3 z-10 flex gap-1.5 p-1 bg-background/60 backdrop-blur border border-border/40 rounded-lg shadow-lg">
          <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => setZoom((z) => Math.min(2, z + 0.1))} title="Zoom In">
            <ZoomIn className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => setZoom((z) => Math.max(0.5, z - 0.1))} title="Zoom Out">
            <ZoomOut className="h-4 w-4" />
          </Button>
          <Button variant="ghost" size="icon" className="h-8 w-8" onClick={resetViewport} title="Reset Center">
            <RotateCcw className="h-4 w-4" />
          </Button>
        </div>

        <svg
          ref={containerRef}
          className="w-full flex-1 bg-background/10 cursor-grab active:cursor-grabbing select-none"
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseUp}
          onMouseDown={handleBgMouseDown}
        >
          {/* Grid background */}
          <defs>
            <pattern id="graph-grid" width="30" height="30" patternUnits="userSpaceOnUse">
              <path d="M 30 0 L 0 0 0 30" fill="none" stroke="rgba(255, 255, 255, 0.03)" strokeWidth="1" />
            </pattern>
          </defs>
          <rect width="100%" height="100%" fill="url(#graph-grid)" />

          <g transform={`translate(${pan.x}, ${pan.y}) scale(${zoom})`}>
            {/* Draw Edges */}
            {edges.map((edge, i) => {
              const source = positions[edge.source];
              const target = positions[edge.target];
              if (!source || !target) return null;

              const isHighlighted = hoveredNode 
                ? (edge.source === hoveredNode || edge.target === hoveredNode) 
                : true;

              return (
                <g key={i}>
                  <line
                    x1={source.x}
                    y1={source.y}
                    x2={target.x}
                    y2={target.y}
                    className="transition-all duration-300"
                    stroke={isHighlighted ? "rgba(96, 165, 250, 0.4)" : "rgba(255, 255, 255, 0.05)"}
                    strokeWidth={isHighlighted ? 1.5 : 1}
                  />
                  {isHighlighted && !hoveredNode && (
                    <text
                      x={(source.x + target.x) / 2}
                      y={(source.y + target.y) / 2 - 4}
                      fill="rgba(148, 163, 184, 0.7)"
                      fontSize="8px"
                      textAnchor="middle"
                      className="font-mono pointer-events-none"
                    >
                      {edge.label || edge.type}
                    </text>
                  )}
                </g>
              );
            })}

            {/* Draw Nodes */}
            {nodes.map((node) => {
              const pos = positions[node.id];
              if (!pos) return null;

              const isHovered = hoveredNode === node.id;
              const isDimmed = hoveredNode && !connectedNodeIds.has(node.id);
              const nodeColorClass = getNodeColorClass(node.type);

              return (
                <g
                  key={node.id}
                  transform={`translate(${pos.x}, ${pos.y})`}
                  className="transition-all duration-200 cursor-pointer"
                  onMouseEnter={() => setHoveredNode(node.id)}
                  onMouseLeave={() => setHoveredNode(null)}
                  onMouseDown={(e) => handleMouseDown(node.id, e)}
                >
                  <circle
                    r={isHovered ? 24 : 18}
                    className={`transition-all duration-300 ${nodeColorClass}`}
                    strokeWidth={isHovered ? 2 : 1}
                  />
                  <circle
                    r={10}
                    className={`fill-background stroke-2 ${nodeColorClass.split(" ")[1]}`}
                  />
                  {React.createElement(getNodeIcon(node.type), {
                    className: `h-3 w-3 absolute pointer-events-none ${nodeColorClass.split(" ")[0]}`,
                    style: { transform: "translate(-6px, -6px)" },
                  })}

                  <g transform="translate(0, 26)">
                    <text
                      fill="white"
                      fontSize={isHovered ? "10px" : "8px"}
                      textAnchor="middle"
                      className="font-medium"
                      opacity={isDimmed ? 0.2 : 0.9}
                    >
                      {node.label.length > 20 ? `${node.label.slice(0, 18)}...` : node.label}
                    </text>
                  </g>
                </g>
              );
            })}
          </g>
        </svg>
      </Card>
    </div>
  );
}
