# Timeline

## Overview

Visual, chronological reconstruction of incident events.

## Technology: D3.js with custom SVG rendering

## Data Model

```typescript
interface TimelineEvent {
  id: string;
  timestamp: string;         // ISO 8601
  type: EventType;
  title: string;
  description?: string;
  source?: string;           // EDR, SIEM, Firewall, etc.
  severity?: Severity;
  evidence_id?: string;      // Link to evidence
  technique_id?: string;     // Link to MITRE technique
  entity_ids?: string[];     // Related entities
}

enum EventType {
  Alert,
  Execution,
  NetworkConnection,
  FileCreation,
  ProcessCreation,
  UserAction,
  AnalystAction,
  Mitigation,
  Observation,
}
```

## Features

### Visual Layout
- Vertical timeline with events as cards
- Events grouped by phase (Initial Access, Execution, etc.)
- Color-coded by event type
- Icons per event type for quick scanning

### Interactions
- **Scroll** — pan through time
- **Click event** — scroll to evidence detail
- **Zoom** — collapse/expand time scale (hour → day → week)
- **Filter** — by event type, source, technique
- **Search** — text search across event titles/descriptions

### Groups
Events auto-grouped by:
- MITRE tactic phase (if techniques mapped)
- Time window (configurable: 1h, 6h, 24h)
- Source system

### Kill Chain View
Toggle to show events arranged by kill chain phase columns:
```
Recon │ Weaponize │ Deliver │ Exploit │ Install │ C2 │ Actions
```
Events placed in the appropriate column based on technique mapping.

## Implementation

```typescript
interface TimelineState {
  events: TimelineEvent[];
  groups: TimelineGroup[];
  zoomLevel: 'hour' | 'day' | 'week';
  selectedEventId: string | null;
  visibleRange: { start: DateTime; end: DateTime };
}

interface TimelineGroup {
  id: string;
  label: string;
  events: TimelineEvent[];
}
```

## API

Timeline data from `GET /api/v1/incidents/{id}/timeline`.

Returns ordered array of `TimelineEvent` objects, optionally grouped by technique/tactic.
