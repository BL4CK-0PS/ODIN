# Graph Explorer

## Overview

Interactive knowledge graph visualization for exploring entity relationships across incidents.

## Technology: D3.js Force-Directed Graph

Using D3.js force simulation for layout.

## Features

### Node Types (Color-Coded)
| Node Type | Color | Shape |
|-----------|-------|-------|
| Incident | Blue | Rounded rect |
| IP Address | Red | Circle |
| Domain | Orange | Diamond |
| Hash | Purple | Hexagon |
| Email | Green | Triangle |
| Hostname | Teal | Square |
| User | Yellow | Circle |
| Technique | Gray | Pentagon |

### Interactions
- **Pan/zoom** — mouse drag + scroll
- **Click node** — open detail panel (right sidebar)
- **Double-click** — navigate to incident/entity page
- **Hover** — highlight connected nodes + edges
- **Drag node** — manual reposition (pins)
- **Search** — find and focus on specific entity
- **Filter** — by type, incident, or technique

### Layout Controls
- **Force-directed** — default, auto-balanced
- **Radial** — center on selected node
- **Hierarchical** — top-down (tactic → technique → incident)
- **Timeline** — nodes positioned by time

### Side Panel (on node select)
Shows:
- Entity detail (type, value, first/last seen)
- Connected incidents (count + list)
- Connected entities (with relationship type)
- Quick actions (copy value, search, navigate)

## Implementation

```typescript
interface GraphNode {
  id: string;
  label: string;
  type: EntityType;
  data: Record<string, unknown>;
  x?: number;
  y?: number;
}

interface GraphEdge {
  id: string;
  source: string;      // node id
  target: string;
  relationship: string; // e.g., "OBSERVED_IN"
  label?: string;
}

interface GraphState {
  nodes: GraphNode[];
  edges: GraphEdge[];
  selectedNodeId: string | null;
  hoveredNodeId: string | null;
  zoom: number;
  viewBox: { x: number; y: number };
}
```

## API Integration

Graph data fetched from `POST /api/v1/graph/path` or `GET /api/v1/incidents/{id}/graph`.

Response includes nodes and edges for the incident's subgraph.
