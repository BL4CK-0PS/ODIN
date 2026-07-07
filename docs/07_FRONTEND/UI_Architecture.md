# UI Architecture

## Stack

- **Framework:** SvelteKit 5 (SSR + client hydration)
- **Language:** TypeScript
- **Styling:** Tailwind CSS 4
- **Visualization:** D3.js (graph), vis.js (timeline)
- **State:** Svelte writable stores + URL search params

## Route Structure

```typescript
src/routes/
├── +layout.svelte          // App shell (sidebar, topbar)
├── +page.svelte            // Dashboard / landing
├── login/
│   └── +page.svelte
├── incidents/
│   ├── +page.svelte        // Incident list (table + filters)
│   ├── [id]/
│   │   ├── +page.svelte    // Incident detail
│   │   ├── overview/       // Summary tab
│   │   ├── timeline/       // Timeline tab
│   │   ├── evidence/       // Evidence tab
│   │   ├── graph/          // Knowledge graph tab
│   │   ├── narrative/      // Narrative tab
│   │   └── similar/        // Similar incidents tab
│   └── new/
│       └── +page.svelte    // Create incident form
├── search/
│   └── +page.svelte        // Full search page
├── graph/
│   └── +page.svelte        // Full-screen graph explorer
├── playbooks/
│   ├── +page.svelte        // Playbook list
│   └── [id]/
│       └── +page.svelte    // Playbook detail/editor
├── settings/
│   └── +page.svelte
└── admin/
    └── +page.svelte        // Workspace admin
```

## Component Tree (Incident Detail)

```
IncidentDetailPage
├── IncidentHeader (title, severity badge, status, assignee)
├── Tabs (overview | timeline | evidence | graph | narrative | similar)
├── TabContent
│   ├── OverviewTab
│   │   ├── TechniqueList (MITRE matrix chips)
│   │   ├── ObservableTable (IOCs with enrichment badges)
│   │   └── SimilarIncidentsPanel (memory matches)
│   ├── TimelineTab
│   │   ├── TimelineCanvas (D3.js)
│   │   └── TimelineToolbar (zoom, filter)
│   ├── EvidenceTab
│   │   ├── EvidenceList (files, notes, artifacts)
│   │   └── EvidenceUploader (drag-drop)
│   ├── GraphTab
│   │   └── GraphExplorer (D3.js force-directed)
│   ├── NarrativeTab
│   │   ├── NarrativeViewer (rendered sections)
│   │   ├── NarrativeEditor (markdown)
│   │   └── ExportActions (PDF, MD, JSON)
│   └── SimilarTab
│       ├── SimilarityScoreBar (animated)
│       ├── MatchedItems (techniques, IOCs)
│       └── SimilarIncidentCards
└── ActivityFeed (sidebar)
```

## Data Flow

```
Page load
  │
  ├── load() function (SvelteKit server load)
  │   ├── Fetch data from API
  │   └── Return as page store
  │
  ├── Client-side hydration
  │   ├── Bind data to components
  │   └── Set up WebSocket for real-time updates
  │
  └── User interaction
      ├── Optimistic UI update (store)
      ├── API call (fetch)
      ├── On success → confirm store update
      └── On failure → rollback + toast error
```
