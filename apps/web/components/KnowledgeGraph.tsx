"use client";

import React, { useState, useEffect, useRef, useMemo, useCallback } from "react";
import * as d3 from "d3";
import dagre from "dagre";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Button } from "@/components/ui/button";
import { Brain, FileText, Globe, Terminal, User, Shield, Network, Minus } from "lucide-react";

export type GraphLayout = "force" | "radial" | "cluster" | "concentric" | "hierarchical";

interface GraphNode { id: string; type: string; label: string; }
interface GraphEdge { source: string; target: string; type?: string; label?: string; }
interface KnowledgeGraphProps {
  nodes: GraphNode[]; edges: GraphEdge[]; allNodeCount?: number; layout?: GraphLayout;
}
interface SimNode extends d3.SimulationNodeDatum { id: string; type: string; label: string; }
type SimLink = d3.SimulationLinkDatum<SimNode> & { type?: string; label?: string; };

const NODE_TYPE_CONFIG: Record<string, { color: string; label: string }> = {
  incident:  { color: "#60A5FA", label: "Incident" },
  evidence:  { color: "#22D3EE", label: "Evidence" },
  ipaddress: { color: "#34D399", label: "IP / Domain" },
  domain:    { color: "#34D399", label: "IP / Domain" },
  process:   { color: "#A78BFA", label: "Process" },
  file:      { color: "#FBBF24", label: "File" },
  user:      { color: "#FB7185", label: "User" },
};
const DEFAULT_COLOR = "#6B7280";

const ICON_PATHS: Record<string, string> = {
  incident: "M0-6l3 5h-4l2 5-5-6h4z",
  evidence: "M-5-5h7l3 3v7H-5zM-2-5v4h4",
  ipaddress: "M0-6a6 6 0 110 12A6 6 0 110-6M-6 0h12M0-6v12",
  process: "M0-4a4 4 0 110 8A4 4 0 110-4",
  file: "M-5-5h5l5 5v5H-5zM0-5v5h5",
  user: "M0-4a3 3 0 110 6A3 3 0 110-4M-5 5c0-3 10-3 10 0",
  shield: "M-5-5l5-3 5 3v4c0 4-5 7-5 7s-5-3-5-7z",
};

const LAYOUT_LABELS: Record<GraphLayout, { label: string; desc: string }> = {
  force:        { label: "Force", desc: "Physics simulation" },
  radial:       { label: "Radial", desc: "Hub-and-spoke by connection distance" },
  cluster:      { label: "Cluster", desc: "Grouped by entity type" },
  concentric:   { label: "Concentric", desc: "Rings by connection count" },
  hierarchical: { label: "Hierarchical", desc: "Ranked top-to-bottom (dagre)" },
};

const LEGEND_ITEMS = ["Incident", "Evidence", "IP / Domain", "Process", "File", "User"];

function getNodeColor(type: string): string {
  return NODE_TYPE_CONFIG[type.toLowerCase()]?.color ?? DEFAULT_COLOR;
}

function bfsDistances(simNodes: SimNode[], simLinks: SimLink[], centerId: string): Map<string, number> {
  const adj = new Map<string, string[]>();
  simNodes.forEach((n) => adj.set(n.id, []));
  simLinks.forEach((l) => {
    const s = typeof l.source === "object" ? (l.source as SimNode).id : String(l.source);
    const t = typeof l.target === "object" ? (l.target as SimNode).id : String(l.target);
    adj.get(s)?.push(t); adj.get(t)?.push(s);
  });
  const dist = new Map<string, number>();
  const queue: string[] = [centerId];
  dist.set(centerId, 0);
  while (queue.length > 0) {
    const cur = queue.shift()!;
    for (const nb of adj.get(cur) || []) {
      if (!dist.has(nb)) { dist.set(nb, dist.get(cur)! + 1); queue.push(nb); }
    }
  }
  simNodes.forEach((n) => { if (!dist.has(n.id)) dist.set(n.id, Infinity); });
  return dist;
}

function computeHierarchicalPositions(simNodes: SimNode[], simLinks: SimLink[], w: number, h: number) {
  const g = new dagre.graphlib.Graph();
  g.setGraph({ rankdir: "TB", nodesep: 60, ranksep: 120, marginx: 40, marginy: 40 });
  g.setDefaultEdgeLabel(() => ({}));
  simNodes.forEach((n) => g.setNode(n.id, { width: 70, height: 70 }));
  simLinks.forEach((l) => {
    const s = typeof l.source === "object" ? (l.source as SimNode).id : String(l.source);
    const t = typeof l.target === "object" ? (l.target as SimNode).id : String(l.target);
    g.setEdge(s, t);
  });
  dagre.layout(g);
  const xMin = Math.min(...simNodes.map((n) => g.node(n.id).x));
  const xMax = Math.max(...simNodes.map((n) => g.node(n.id).x));
  const yMin = Math.min(...simNodes.map((n) => g.node(n.id).y));
  const yMax = Math.max(...simNodes.map((n) => g.node(n.id).y));
  const graphW = Math.max(xMax - xMin, 1);
  const graphH = Math.max(yMax - yMin, 1);
  const scale = Math.min((w * 0.8) / graphW, (h * 0.8) / graphH);
  const ox = (w - graphW * scale) / 2 - xMin * scale;
  const oy = (h - graphH * scale) / 2 - yMin * scale;
  const ranks = new Map<number, number>();
  simNodes.forEach((n) => {
    const node = g.node(n.id);
    n.x = node.x * scale + ox;
    n.y = node.y * scale + oy;
    const rank = (node as unknown as { rank?: number }).rank ?? 0;
    ranks.set(rank, (ranks.get(rank) || 0) + 1);
  });
  return { ranks, scale, ox, oy };
}

export function KnowledgeGraph({ nodes, edges, allNodeCount, layout = "force" }: KnowledgeGraphProps) {
  const svgRef = useRef<SVGSVGElement>(null);
  const wrapperRef = useRef<HTMLDivElement>(null);
  const zoomRef = useRef<d3.ZoomBehavior<SVGSVGElement, unknown> | null>(null);
  const simRef = useRef<d3.Simulation<SimNode, SimLink> | null>(null);
  const posCacheRef = useRef<Map<string, { x: number; y: number }>>(new Map());
  const selectedIdRef = useRef<string | null>(null);

  const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null);
  const [zoomLevel, setZoomLevel] = useState(1);

  const degree = useMemo(() => {
    const counts: Record<string, number> = {};
    nodes.forEach((n) => { counts[n.id] = 0; });
    edges.forEach((e) => {
      if (counts[e.source] !== undefined) counts[e.source]++;
      if (counts[e.target] !== undefined) counts[e.target]++;
    });
    return counts;
  }, [nodes, edges]);

  const nodeMap = useMemo(() => {
    const map = new Map<string, GraphNode>();
    nodes.forEach((n) => map.set(n.id, n));
    return map;
  }, [nodes]);

  const maxDeg = useMemo(() => Math.max(1, ...Object.values(degree)), [degree]);
  const getNodeRadius = useCallback((deg: number) => 12 + (deg / maxDeg) * 20, [maxDeg]);

  useEffect(() => {
    const el = svgRef.current;
    const wrapper = wrapperRef.current;
    if (!el || !wrapper || nodes.length === 0) return;

    const width = wrapper.clientWidth;
    const height = wrapper.clientHeight;
    const cx = width / 2;
    const cy = height / 2;
    const boundary = Math.min(width, height) * 0.35;

    const simNodes: SimNode[] = nodes.map((n) => ({ id: n.id, type: n.type, label: n.label }));
    const nodeIds = new Set(simNodes.map((n) => n.id));
    const simLinks: SimLink[] = edges
      .filter((e) => nodeIds.has(e.source as string) && nodeIds.has(e.target as string))
      .map((e) => ({ source: e.source as string, target: e.target as string, type: e.type, label: e.label }));

    const svg = d3.select(el);
    svg.selectAll("*").remove();

    const defs = svg.append("defs");

    defs.append("pattern")
      .attr("id", "g-grid").attr("width", 30).attr("height", 30)
      .attr("patternUnits", "userSpaceOnUse")
      .append("path").attr("d", "M 30 0 L 0 0 0 30")
      .attr("fill", "none").attr("stroke", "rgba(255,255,255,0.03)").attr("strokeWidth", 1);

    const markerId = `arrow-${layout}`;
    defs.append("marker")
      .attr("id", markerId).attr("viewBox", "0 -5 10 10")
      .attr("refX", 22).attr("refY", 0)
      .attr("markerWidth", 6).attr("markerHeight", 6)
      .attr("orient", "auto-start-reverse")
      .append("path").attr("d", "M 0 -4 L 10 0 L 0 4")
      .attr("fill", "rgba(148,163,184,0.25)");

    const gradientId = `g-grad-${Math.random().toString(36).slice(2)}`;
    defs.append("radialGradient").attr("id", gradientId)
      .attr("cx", "50%").attr("cy", "50%").attr("r", "50%")
      .selectAll("stop").data(["rgba(249,115,22,0)", "rgba(249,115,22,0.12)", "rgba(249,115,22,0)"])
      .join("stop").attr("offset", (_, i) => [`0%`, `50%`, `100%`][i])
      .attr("stop-color", (d) => d);

    svg.append("rect").attr("width", "100%").attr("height", "100%").attr("fill", "url(#g-grid)");

    const g = svg.append("g");
    const guidesGroup = g.append("g").attr("class", "guides").attr("pointer-events", "none");
    const linkGroup = g.append("g").attr("class", "links");
    const nodeGroup = g.append("g").attr("class", "nodes");
    const labelGroup = g.append("g").attr("class", "labels");

    const colorMap: Record<string, string> = {};
    simNodes.forEach((n) => { colorMap[n.id] = getNodeColor(n.type); });

    simNodes.forEach((n) => {
      const cached = posCacheRef.current.get(n.id);
      if (cached) { n.x = cached.x; n.y = cached.y; }
      else { n.x = cx + (Math.random() - 0.5) * 200; n.y = cy + (Math.random() - 0.5) * 200; }
    });

    let hierarchicalRanks: Map<number, number> = new Map();

    if (layout === "hierarchical") {
      hierarchicalRanks = computeHierarchicalPositions(simNodes, simLinks, width, height).ranks;
    }

    if (layout === "radial") {
      let maxDegNode = simNodes[0].id;
      let maxD = 0;
      simNodes.forEach((n) => { if ((degree[n.id] || 0) > maxD) { maxD = degree[n.id] || 0; maxDegNode = n.id; } });
      const dists = bfsDistances(simNodes, simLinks, maxDegNode);
      const maxDist = Math.max(1, ...Array.from(dists.values()).filter((d) => d < Infinity));
      const ringCounts = new Map<number, number>();
      const ringIndexed = new Map<number, number>();
      simNodes.forEach((n) => {
        const d = dists.get(n.id) ?? maxDist;
        const ring = d === Infinity ? maxDist + 1 : d;
        ringCounts.set(ring, (ringCounts.get(ring) || 0) + 1);
      });
      simNodes.forEach((n) => {
        const d = dists.get(n.id) ?? maxDist;
        const ring = d === Infinity ? maxDist + 1 : d;
        const idx = ringIndexed.get(ring) || 0;
        ringIndexed.set(ring, idx + 1);
        const total = ringCounts.get(ring) || 1;
        const angle = (idx / total) * 2 * Math.PI - Math.PI / 2;
        n.x = cx + Math.cos(angle) * boundary * (ring / (maxDist + 1));
        n.y = cy + Math.sin(angle) * boundary * (ring / (maxDist + 1));
      });
      for (let r = 1; r <= maxDist + 1; r++) {
        const rr = boundary * (r / (maxDist + 1));
        guidesGroup.append("circle").attr("cx", cx).attr("cy", cy).attr("r", rr)
          .attr("fill", "none").attr("stroke", "rgba(255,255,255,0.04)")
          .attr("strokeWidth", 1).attr("stroke-dasharray", "4,5");
        guidesGroup.append("text").attr("x", cx + rr + 4).attr("y", cy + 3)
          .attr("fill", "rgba(255,255,255,0.1)").attr("fontSize", 7).text(`hop ${r}`);
      }
      guidesGroup.append("circle").attr("cx", cx).attr("cy", cy).attr("r", 8)
        .attr("fill", "rgba(249,115,22,0.15)").attr("stroke", "rgba(249,115,22,0.3)").attr("strokeWidth", 1);
    }

    if (layout === "concentric") {
      const sorted = [...simNodes].sort((a, b) => (degree[b.id] || 0) - (degree[a.id] || 0));
      const perRing = Math.ceil(sorted.length / 3);
      sorted.forEach((n, i) => {
        const ring = Math.min(Math.floor(i / perRing), 2);
        const idx = i % perRing;
        const total = Math.min(perRing, sorted.length - ring * perRing);
        const angle = (idx / total) * 2 * Math.PI - Math.PI / 2;
        n.x = cx + Math.cos(angle) * boundary * ((ring + 1) / 3);
        n.y = cy + Math.sin(angle) * boundary * ((ring + 1) / 3);
      });
      ["highest degree", "", "lowest degree"].forEach((label, r) => {
        const rr = boundary * ((r + 1) / 3);
        guidesGroup.append("circle").attr("cx", cx).attr("cy", cy).attr("r", rr)
          .attr("fill", "none").attr("stroke", "rgba(255,255,255,0.04)")
          .attr("strokeWidth", 1).attr("stroke-dasharray", "4,5");
        guidesGroup.append("text").attr("x", cx + rr + 4).attr("y", cy + 3)
          .attr("fill", "rgba(255,255,255,0.1)").attr("fontSize", 6).text(label);
      });
    }

    if (layout === "cluster") {
      const groups = new Map<string, SimNode[]>();
      simNodes.forEach((n) => {
        const key = n.type.toLowerCase();
        if (!groups.has(key)) groups.set(key, []);
        groups.get(key)!.push(n);
      });
      const entries = Array.from(groups.entries());
      const cols = Math.ceil(Math.sqrt(entries.length));
      const cellW = (width * 0.7) / Math.max(cols, 1);
      const cellH = (height * 0.7) / Math.ceil(entries.length / cols);
      entries.forEach(([type, group], i) => {
        const col = i % cols, row = Math.floor(i / cols);
        const gcx = width * 0.15 + col * cellW + cellW / 2;
        const gcy = height * 0.15 + row * cellH + cellH / 2;
        const spread = Math.min(cellW, cellH) * 0.35;
        group.forEach((n, j) => {
          const angle = (j / group.length) * 2 * Math.PI;
          n.x = gcx + Math.cos(angle) * spread;
          n.y = gcy + Math.sin(angle) * spread;
        });
        const gc = getNodeColor(type);
        guidesGroup.append("rect")
          .attr("x", gcx - cellW / 2 + 15).attr("y", gcy - cellH / 2 + 15)
          .attr("width", cellW - 30).attr("height", cellH - 30)
          .attr("rx", 10).attr("fill", `${gc}08`).attr("stroke", `${gc}18`)
          .attr("strokeWidth", 1).attr("stroke-dasharray", "3,3");
        guidesGroup.append("text").attr("x", gcx).attr("y", gcy - cellH / 2 + 27)
          .attr("fill", gc).attr("fontSize", 8).attr("fontWeight", "bold")
          .attr("textAnchor", "middle")
          .text(type.charAt(0).toUpperCase() + type.slice(1));
      });
    }

    if (layout === "hierarchical") {
      const yPositions = simNodes.map((n) => n.y ?? 0);
      const ySorted = Array.from(new Set(yPositions)).sort((a, b) => a - b);
      const spacing = ySorted.length > 1 ? ySorted[1] - ySorted[0] : 100;
      ySorted.forEach((yPos, i) => {
        if (i > 0) {
          guidesGroup.append("line").attr("x1", width * 0.05).attr("x2", width * 0.95)
            .attr("y1", yPos - spacing / 2).attr("y2", yPos - spacing / 2)
            .attr("stroke", "rgba(255,255,255,0.04)").attr("strokeWidth", 1)
            .attr("stroke-dasharray", "2,4");
        }
        guidesGroup.append("text").attr("x", width * 0.03).attr("y", yPos + 3)
          .attr("fill", "rgba(255,255,255,0.1)").attr("fontSize", 7).text(`L${i + 1}`);
      });
    }

    const simulation = d3.forceSimulation(simNodes)
      .force("link", d3.forceLink<SimNode, SimLink>(simLinks).id((d) => d.id).distance(120).strength(0.3))
      .force("center", d3.forceCenter(cx, cy))
      .alphaDecay(0.04).velocityDecay(0.35);

    if (layout === "force") {
      simulation.force("charge", d3.forceManyBody().strength(-500));
      simulation.force("collide", d3.forceCollide<SimNode>().radius((d) => getNodeRadius(degree[d.id] || 0) + 8));
    }

    if (layout === "cluster") {
      const typeList = Array.from(new Set(simNodes.map((n) => n.type.toLowerCase())));
      const cols = Math.ceil(Math.sqrt(typeList.length));
      const cellW = (width * 0.7) / Math.max(cols, 1);
      const cellH = (height * 0.7) / Math.ceil(typeList.length / cols);
      typeList.forEach((type, i) => {
        const col = i % cols, row = Math.floor(i / cols);
        const gcx = width * 0.15 + col * cellW + cellW / 2;
        const gcy = height * 0.15 + row * cellH + cellH / 2;
        simulation.force(`x-${type}`, d3.forceX(gcx).strength(0.12));
        simulation.force(`y-${type}`, d3.forceY(gcy).strength(0.12));
      });
      simulation.force("charge", d3.forceManyBody().strength(-150));
      simulation.force("collide", d3.forceCollide<SimNode>().radius((d) => getNodeRadius(degree[d.id] || 0) + 4));
    }

    if (layout === "radial" || layout === "concentric") {
      simulation.force("charge", d3.forceManyBody().strength(-80));
      simulation.force("collide", d3.forceCollide<SimNode>().radius((d) => getNodeRadius(degree[d.id] || 0) + 4));
    }

    if (layout === "hierarchical") {
      simulation.alphaDecay(0.15).velocityDecay(0.9);
      simulation.force("charge", d3.forceManyBody().strength(-30));
      simulation.force("collide", d3.forceCollide<SimNode>().radius((d) => getNodeRadius(degree[d.id] || 0) + 4));
      simulation.force("x", d3.forceX(cx).strength(0.01));
      simulation.force("y", d3.forceY(cy).strength(0.005));
    }

    simRef.current = simulation;

    const useCurvedEdges = layout === "force";
    const edgeStrokeBase = layout === "cluster" ? "rgba(148,163,184,0.1)" : "rgba(148,163,184,0.15)";

    const link = linkGroup.selectAll<SVGGElement, SimLink>("g")
      .data(simLinks).join("g").attr("class", "link-group");

    if (useCurvedEdges) {
      link.append("path")
        .attr("class", "link-line")
        .attr("fill", "none")
        .attr("stroke", edgeStrokeBase)
        .attr("strokeWidth", 1)
        .attr("marker-end", `url(#${markerId})`);
    } else {
      link.append("line")
        .attr("class", "link-line")
        .attr("stroke", edgeStrokeBase)
        .attr("strokeWidth", 1)
        .attr("marker-end", `url(#${markerId})`);
    }

    link.append("text")
      .attr("fill", "rgba(148,163,184,0.3)")
      .attr("fontSize", 6).attr("textAnchor", "middle").attr("dy", "-4")
      .attr("class", "link-label font-mono")
      .text((d) => d.label || d.type || "connected_to");

    const node = nodeGroup.selectAll<SVGGElement, SimNode>("g")
      .data(simNodes).join("g")
      .attr("class", "node-group").style("cursor", "pointer");

    if (layout === "cluster") {
      const rFn = (d: SimNode) => getNodeRadius(degree[d.id] || 0);
      node.append("rect").attr("class", "node-outer")
        .attr("x", (d) => -rFn(d) - 10).attr("y", (d) => -rFn(d) - 4)
        .attr("width", (d) => (rFn(d) + 10) * 2).attr("height", (d) => (rFn(d) + 4) * 2)
        .attr("rx", 10)
        .attr("fill", (d) => { const c = NODE_TYPE_CONFIG[d.type.toLowerCase()]; return c ? `${c.color}18` : `${DEFAULT_COLOR}18`; })
        .attr("stroke", (d) => colorMap[d.id]).attr("strokeWidth", 1.5);
      node.append("rect").attr("class", "node-inner")
        .attr("x", (d) => -rFn(d) * 0.35).attr("y", (d) => -rFn(d) * 0.35)
        .attr("width", (d) => rFn(d) * 0.7).attr("height", (d) => rFn(d) * 0.7)
        .attr("rx", 6).attr("fill", "hsl(var(--background))")
        .attr("stroke", (d) => colorMap[d.id]).attr("strokeWidth", 1.5);
      node.append("path").attr("class", "node-icon")
        .attr("d", (d) => ICON_PATHS[d.type.toLowerCase()] || ICON_PATHS.shield)
        .attr("fill", (d) => colorMap[d.id]).attr("stroke", (d) => colorMap[d.id]).attr("strokeWidth", 1)
        .attr("transform", (d) => { const s = Math.min(rFn(d) * 0.03, 0.5); return `translate(${-rFn(d) * 0.35},0) scale(${Math.max(s, 0.25)})`; });
      node.append("text").attr("class", "node-cluster-type")
        .attr("fill", (d) => colorMap[d.id]).attr("fontSize", 6).attr("fontWeight", "bold")
        .attr("x", (d) => rFn(d) * 0.2).attr("y", 0).attr("dy", 2).attr("textAnchor", "start")
        .text((d) => d.type);
    } else if (layout === "hierarchical") {
      const rFn = (d: SimNode) => Math.max(getNodeRadius(degree[d.id] || 0), 16);
      const lw = (d: SimNode) => Math.max(rFn(d) * 2 + 12, d.label.length * 5.5 + 20);
      node.append("rect").attr("class", "node-outer")
        .attr("x", (d) => -lw(d) / 2).attr("y", -12)
        .attr("width", (d) => lw(d)).attr("height", 24).attr("rx", 8)
        .attr("fill", "hsl(var(--card))").attr("stroke", (d) => colorMap[d.id]).attr("strokeWidth", 1.5);
      node.append("circle").attr("class", "node-inner")
        .attr("r", (d) => rFn(d) * 0.35)
        .attr("fill", (d) => { const c = NODE_TYPE_CONFIG[d.type.toLowerCase()]; return c ? c.color : DEFAULT_COLOR; })
        .attr("cx", (d) => -lw(d) / 2 + rFn(d) * 0.5 + 4).attr("stroke", "none");
      node.append("path").attr("class", "node-icon")
        .attr("d", (d) => ICON_PATHS[d.type.toLowerCase()] || ICON_PATHS.shield)
        .attr("fill", "hsl(var(--background))").attr("stroke", "hsl(var(--background))").attr("strokeWidth", 1)
        .attr("transform", (d) => {
          const xOff = -lw(d) / 2 + rFn(d) * 0.5 + 4;
          const s = Math.min(rFn(d) * 0.025, 0.4);
          return `translate(${xOff},0) scale(${Math.max(s, 0.2)})`;
        });
      node.append("text").attr("class", "node-hier-label")
        .attr("fill", "rgba(255,255,255,0.85)").attr("fontSize", 7).attr("fontWeight", "medium")
        .attr("textAnchor", "middle").attr("dy", 3)
        .text((d) => d.label.length > 18 ? d.label.slice(0, 16) + ".." : d.label);
    } else {
      const rFn = (d: SimNode) => getNodeRadius(degree[d.id] || 0);
      node.append("circle").attr("class", "node-outer")
        .attr("r", (d) => rFn(d))
        .attr("fill", (d) => { const c = NODE_TYPE_CONFIG[d.type.toLowerCase()]; return c ? `${c.color}20` : `${DEFAULT_COLOR}20`; })
        .attr("stroke", (d) => colorMap[d.id]).attr("strokeWidth", 1.5);
      node.append("circle").attr("class", "node-inner")
        .attr("r", (d) => rFn(d) * 0.45).attr("fill", "hsl(var(--background))")
        .attr("stroke", (d) => colorMap[d.id]).attr("strokeWidth", 1.5);
      node.append("path").attr("class", "node-icon")
        .attr("d", (d) => ICON_PATHS[d.type.toLowerCase()] || ICON_PATHS.shield)
        .attr("fill", (d) => colorMap[d.id]).attr("stroke", (d) => colorMap[d.id]).attr("strokeWidth", 1)
        .attr("transform", (d) => { const s = Math.min(rFn(d) * 0.04, 0.6); return `scale(${Math.max(s, 0.3)})`; });
    }

    if (layout === "radial") {
      node.append("path").attr("class", "node-orbital")
        .attr("d", (d) => { const r = getNodeRadius(degree[d.id] || 0) + 5; return `M${-r}0 A${r} ${r} 0 0 1 0 ${-r}`; })
        .attr("fill", "none").attr("stroke", (d) => colorMap[d.id])
        .attr("strokeWidth", 1).attr("stroke-dasharray", "2,3").attr("opacity", 0.35);
    }

    if (layout === "concentric") {
      [0.6, 0.78].forEach((s, i) => {
        node.append("circle").attr("class", `node-ring-${i + 1}`)
          .attr("r", (d) => getNodeRadius(degree[d.id] || 0) * s)
          .attr("fill", "none").attr("stroke", (d) => colorMap[d.id])
          .attr("strokeWidth", 0.5).attr("opacity", 0.35);
      });
    }

    node.style("opacity", 0)
      .transition().duration(400)
      .delay((_, i) => Math.min(i * 6, 250))
      .style("opacity", 1);

    const useSeparateLabels = layout !== "cluster" && layout !== "hierarchical";
    let label: d3.Selection<SVGTextElement, SimNode, SVGGElement, unknown> | null = null;

    if (useSeparateLabels) {
      label = labelGroup.selectAll<SVGTextElement, SimNode>("text")
        .data(simNodes).join("text")
        .attr("fill", "rgba(255,255,255,0.8)").attr("fontSize", 8).attr("textAnchor", "middle")
        .attr("class", "node-label font-medium pointer-events-none")
        .text((d) => (d.label.length > 22 ? d.label.slice(0, 20) + "..." : d.label));
    }

    const hoverableClasses = ".node-outer,.node-inner,.node-icon,.node-orbital,.node-ring-1,.node-ring-2,.node-cluster-type,.node-hier-label";

    node.on("mouseenter", function (event, d) {
      const connected = new Set([d.id]);
      simLinks.forEach((l) => {
        const sid = typeof l.source === "object" ? (l.source as SimNode).id : String(l.source);
        const tid = typeof l.target === "object" ? (l.target as SimNode).id : String(l.target);
        if (sid === d.id) connected.add(tid);
        if (tid === d.id) connected.add(sid);
      });
      nodeGroup.selectAll<SVGGElement, SimNode>("g").each(function (nd) {
        const el = d3.select(this);
        const isConn = connected.has(nd.id);
        el.selectAll(hoverableClasses).transition().duration(200).attr("opacity", isConn ? 1 : 0.15);
      });
      labelGroup.selectAll("text").transition().duration(200).attr("opacity", (nd: unknown) => connected.has((nd as SimNode).id) ? 1 : 0.15);
      linkGroup.selectAll<SVGGElement, SimLink>("g").each(function (ld) {
        const sid = typeof ld.source === "object" ? (ld.source as SimNode).id : ld.source;
        const tid = typeof ld.target === "object" ? (ld.target as SimNode).id : ld.target;
        const isConn = sid === d.id || tid === d.id;
        d3.select(this).select(".link-line")
          .transition().duration(200)
          .attr("stroke", isConn ? "rgba(96,165,250,0.5)" : edgeStrokeBase)
          .attr("strokeWidth", isConn ? 2 : 1).attr("opacity", isConn ? 1 : 0.2);
        d3.select(this).select("text")
          .transition().duration(200)
          .attr("fill", isConn ? "rgba(148,163,184,0.8)" : "rgba(148,163,184,0.08)");
      });
    });

    node.on("mouseleave", function () {
      nodeGroup.selectAll<SVGGElement, SimNode>("g").each(function () {
        d3.select(this).selectAll(hoverableClasses).transition().duration(300).attr("opacity", 1);
      });
      labelGroup.selectAll("text").transition().duration(300).attr("opacity", 1);
      linkGroup.selectAll<SVGGElement, SimLink>("g").each(function () {
        d3.select(this).select(".link-line")
          .transition().duration(300).attr("stroke", edgeStrokeBase).attr("strokeWidth", 1).attr("opacity", 1);
        d3.select(this).select("text").transition().duration(300).attr("fill", "rgba(148,163,184,0.3)");
      });
    });

    node.on("click", function (event, d) {
      const found = nodes.find((n) => n.id === d.id);
      setSelectedNode(found || null);
      selectedIdRef.current = d.id;

      nodeGroup.selectAll<SVGGElement, SimNode>("g").each(function (nd) {
        const isSel = nd.id === d.id;
        d3.select(this).select(".node-outer")
          .transition().duration(300)
          .attr("strokeWidth", isSel ? 3 : 1.5)
          .attr("stroke", isSel ? "#F97316" : colorMap[nd.id]);
      });

      nodeGroup.selectAll<SVGGElement, SimNode>("g").filter(function (nd) { return nd.id === d.id; })
        .each(function () {
          const sel = d3.select(this);
          sel.selectAll(".pulse-ring").remove();
          sel.append("circle").attr("class", "pulse-ring")
            .attr("r", getNodeRadius(degree[d.id] || 0) + 4)
            .attr("fill", "none").attr("stroke", "#F97316").attr("strokeWidth", 2)
            .attr("opacity", 0.6)
            .transition().duration(1200).ease(d3.easeExpOut)
            .attr("r", getNodeRadius(degree[d.id] || 0) + 18)
            .attr("opacity", 0)
            .remove();
        });
    });

    const drag = d3.drag<SVGGElement, SimNode>()
      .on("start", function (event, d) {
        if (!event.active) simulation.alphaTarget(0.3).restart();
        d.fx = d.x; d.fy = d.y;
      })
      .on("drag", function (event, d) { d.fx = event.x; d.fy = event.y; })
      .on("end", function (event, d) {
        if (!event.active) simulation.alphaTarget(0);
        d.fx = null; d.fy = null;
      });

    node.call(drag);

    const zoom = d3.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.15, 5])
      .on("zoom", (event) => { g.attr("transform", event.transform.toString()); setZoomLevel(event.transform.k); });

    zoomRef.current = zoom;
    svg.call(zoom);

    simulation.on("tick", () => {
      if (useCurvedEdges) {
        link.select(".link-line").attr("d", (d) => {
          const s = d.source as SimNode; const t = d.target as SimNode;
          const sx = s.x ?? 0, sy = s.y ?? 0, tx = t.x ?? 0, ty = t.y ?? 0;
          const dx = tx - sx, dy = ty - sy;
          const cx2 = (sx + tx) / 2 + dy * 0.08;
          const cy2 = (sy + ty) / 2 - dx * 0.08;
          return `M${sx},${sy}Q${cx2},${cy2}${tx},${ty}`;
        });
      } else {
        link.select(".link-line")
          .attr("x1", (d) => ((d.source as SimNode).x ?? 0))
          .attr("y1", (d) => ((d.source as SimNode).y ?? 0))
          .attr("x2", (d) => ((d.target as SimNode).x ?? 0))
          .attr("y2", (d) => ((d.target as SimNode).y ?? 0));
      }
      link.select("text")
        .attr("x", (d) => (((d.source as SimNode).x ?? 0) + ((d.target as SimNode).x ?? 0)) / 2)
        .attr("y", (d) => (((d.source as SimNode).y ?? 0) + ((d.target as SimNode).y ?? 0)) / 2 - 4);
      node.attr("transform", (d) => `translate(${d.x},${d.y})`);
      if (label) {
        label.attr("x", (d) => d.x ?? 0).attr("y", (d) => (d.y ?? 0) + getNodeRadius(degree[d.id] || 0) + 8);
      }
    });

    return () => {
      simNodes.forEach((n) => { posCacheRef.current.set(n.id, { x: n.x ?? 0, y: n.y ?? 0 }); });
      simulation.stop();
    };
  }, [nodes, edges, degree, getNodeRadius, layout]);

  const handleZoomIn = useCallback(() => {
    const el = svgRef.current;
    const zoom = zoomRef.current;
    if (el && zoom) d3.select(el).transition().duration(300).call(zoom.scaleBy, 1.3);
  }, []);
  const handleZoomOut = useCallback(() => {
    const el = svgRef.current;
    const zoom = zoomRef.current;
    if (el && zoom) d3.select(el).transition().duration(300).call(zoom.scaleBy, 0.7);
  }, []);
  const resetViewport = useCallback(() => {
    const el = svgRef.current;
    const zoom = zoomRef.current;
    if (el && zoom) d3.select(el).transition().duration(500).call(zoom.transform, d3.zoomIdentity);
  }, []);

  const layoutInfo = LAYOUT_LABELS[layout];

  return (
    <div className="grid grid-cols-4 gap-4" style={{ height: "70vh" }}>
      <Card className="col-span-1 glass flex flex-col h-full overflow-hidden">
        <CardHeader className="pb-3 border-b border-border/40">
          <div className="flex items-center gap-2">
            <Brain className="h-5 w-5 text-primary" />
            <CardTitle className="text-base font-semibold">Node Inspector</CardTitle>
          </div>
        </CardHeader>
        <CardContent className="flex-1 p-3 overflow-hidden">
          {selectedNode ? (
            <ScrollArea className="h-full pr-1">
              <div className="space-y-4">
                <div>
                  <span className="text-[10px] uppercase font-bold tracking-wider text-muted-foreground block mb-1">Entity Type</span>
                  <span className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium bg-primary/10 border border-primary/20 text-primary">{selectedNode.type}</span>
                </div>
                <div>
                  <span className="text-[10px] uppercase font-bold tracking-wider text-muted-foreground block mb-1">Label</span>
                  <p className="text-sm font-semibold break-words leading-relaxed text-foreground">{selectedNode.label}</p>
                </div>
                <div>
                  <span className="text-[10px] uppercase font-bold tracking-wider text-muted-foreground block mb-1">Connections</span>
                  <span className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium bg-muted/30 border border-border/40">
                    <Network className="h-3 w-3" />
                    {degree[selectedNode.id] || 0} edges
                  </span>
                </div>
                <div>
                  <span className="text-[10px] uppercase font-bold tracking-wider text-muted-foreground block mb-2">Direct Connections</span>
                  <div className="space-y-2">
                    {edges.filter((e) => e.source === selectedNode.id || e.target === selectedNode.id).map((edge, i) => {
                      const targetId = edge.source === selectedNode.id ? edge.target : edge.source;
                      const targetNode = nodeMap.get(targetId);
                      const rel = edge.label || edge.type || "connected_to";
                      return (
                        <div key={i} className="text-xs p-2 rounded bg-muted/30 border border-border/20 flex flex-col gap-1 cursor-pointer hover:bg-muted/50 transition-colors"
                          onClick={() => setSelectedNode(targetNode || null)}>
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
              <p className="text-xs text-muted-foreground leading-relaxed">Click a node to inspect entity attributes and relationships.</p>
            </div>
          )}
        </CardContent>
      </Card>

      <Card className="col-span-3 glass relative h-full overflow-hidden flex flex-col">
        <div ref={wrapperRef} className="absolute inset-0">
          <div className="absolute top-3 right-3 z-10 flex items-center gap-1.5 p-1 bg-background/60 backdrop-blur border border-border/40 rounded-lg shadow-lg">
            <span className="text-[10px] font-mono text-muted-foreground px-1 min-w-[32px] text-center">{Math.round(zoomLevel * 100)}%</span>
            <span className="w-px h-4 bg-border/40" />
            <Button variant="ghost" size="icon" className="h-8 w-8" onClick={handleZoomIn} title="Zoom In">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="h-4 w-4"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/><line x1="11" y1="8" x2="11" y2="14"/><line x1="8" y1="11" x2="14" y2="11"/></svg>
            </Button>
            <Button variant="ghost" size="icon" className="h-8 w-8" onClick={handleZoomOut} title="Zoom Out">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="h-4 w-4"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/><line x1="8" y1="11" x2="14" y2="11"/></svg>
            </Button>
            <Button variant="ghost" size="icon" className="h-8 w-8" onClick={resetViewport} title="Reset View">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="h-4 w-4"><polyline points="1 4 1 10 7 10"/><polyline points="23 20 23 14 17 14"/><path d="M20.49 9A9 9 0 0 0 5.64 5.64L1 10m22 4l-4.64 4.36A9 9 0 0 1 3.51 15"/></svg>
            </Button>
          </div>

          <div className="absolute top-3 left-3 z-10 bg-background/40 backdrop-blur px-2.5 py-1.5 rounded-lg border border-border/30 shadow-sm">
            <span className="text-[11px] font-semibold text-muted-foreground">{layoutInfo.label}</span>
            <span className="text-[10px] text-muted-foreground/60 ml-2 hidden md:inline">{layoutInfo.desc}</span>
          </div>

          {nodes.length > 0 && (
            <div className="absolute bottom-3 left-3 z-10 flex flex-col gap-1 p-2 bg-background/60 backdrop-blur border border-border/40 rounded-lg shadow-lg text-[10px]">
              <span className="font-semibold text-muted-foreground mb-1">Legend</span>
              {LEGEND_ITEMS.map((item) => {
                const cfg = NODE_TYPE_CONFIG[item.toLowerCase()] || NODE_TYPE_CONFIG.incident;
                return (
                  <div key={item} className="flex items-center gap-1.5">
                    <svg width="10" height="10" viewBox="0 0 10 10">
                      <circle cx="5" cy="5" r="4" fill={`${cfg.color}30`} stroke={cfg.color} strokeWidth="1" />
                    </svg>
                    <span className="text-muted-foreground">{item}</span>
                  </div>
                );
              })}
            </div>
          )}

          <div className="absolute bottom-3 right-3 z-10 flex items-center gap-2 text-[10px] text-muted-foreground bg-background/40 backdrop-blur px-2 py-1 rounded border border-border/20">
            <span>{nodes.length} nodes</span>
            <Minus className="h-2.5 w-2.5" />
            <span>{edges.length} edges</span>
            {allNodeCount !== undefined && allNodeCount > nodes.length && (
              <span className="text-blue-400 font-medium">({allNodeCount - nodes.length} filtered)</span>
            )}
          </div>

          <svg ref={svgRef} className="w-full h-full" style={{ display: "block" }} />
        </div>
      </Card>
    </div>
  );
}
