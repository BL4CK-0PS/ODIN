# State Management

## Store Types

```typescript
// src/lib/stores/
├── incidents.ts            // Current incident list + pagination
├── currentIncident.ts      // Open incident detail data
├── auth.ts                 // JWT token, user info
├── ui.ts                   // Sidebar open, active tab, theme
├── search.ts               // Search query + results
├── notifications.ts        // WebSocket events → toast queue
└── graph.ts                // Graph state (selected node, zoom level)
```

## Incident Store (example)

```typescript
// src/lib/stores/incidents.ts
import { writable, derived } from 'svelte/store';

interface IncidentFilters {
  status: string[];
  severity: string[];
  technique: string[];
  search: string;
  page: number;
  perPage: number;
}

const incidents = writable<Incident[]>([]);
const filters = writable<IncidentFilters>(defaultFilters);
const pagination = writable<PaginationMeta>({ page: 1, total: 0, totalPages: 0 });

const filteredIncidents = derived(
  [incidents, filters],
  ([$incidents, $filters]) => {
    // Client-side filtering for instant UI, but primary filtering happens server-side
    return applyFilters($incidents, $filters);
  }
);
```

## WebSocket State Sync

- WebSocket connection established on login
- Events: `incident.created`, `incident.updated`, `narrative.generated`
- On event → update relevant store (optimistic)
- For critical updates, re-verify via API on next page focus

## URL State

- Filters, page number, active tab stored in URL search params
- On param change → update store → trigger API reload
- Enables shareable URLs and browser back/forward

```typescript
// Example URL: /incidents?status=open&severity=critical&page=2
$page.url.searchParams.get('status')  // 'open'
```
