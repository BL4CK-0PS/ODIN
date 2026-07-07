# Components

## Shared UI Components

```
src/lib/components/
├── ui/                          # Generic UI primitives
│   ├── Button.svelte
│   ├── Input.svelte
│   ├── Select.svelte
│   ├── Badge.svelte             # Severity, status, technique badges
│   ├── Card.svelte
│   ├── Modal.svelte
│   ├── Toast.svelte             # Success/error notifications
│   ├── Spinner.svelte
│   ├── EmptyState.svelte
│   ├── Pagination.svelte
│   └── ConfirmDialog.svelte
│
├── incident/                    # Incident-specific components
│   ├── IncidentTable.svelte
│   ├── IncidentCard.svelte
│   ├── SeverityBadge.svelte
│   ├── StatusBadge.svelte
│   ├── StatusTransition.svelte  # Dropdown for status changes
│   ├── TechniqueChips.svelte    # MITRE technique tag list
│   ├── ObservableRow.svelte     # IOC row with enrichment
│   └── SimilarityScore.svelte   # Animated score bar
│
├── narrative/                   # Narrative components
│   ├── NarrativeViewer.svelte
│   ├── NarrativeEditor.svelte
│   └── ExportButton.svelte
│
├── graph/                       # Graph components
│   ├── GraphCanvas.svelte       # D3.js force-directed graph
│   ├── GraphControls.svelte     # Zoom, search, filter
│   └── GraphNodeDetail.svelte   # Side panel on node click
│
├── timeline/                    # Timeline components
│   ├── TimelineCanvas.svelte    # D3.js timeline
│   ├── TimelineEvent.svelte     # Individual event block
│   └── TimelineControls.svelte  # Zoom, group by
│
└── search/                      # Search components
    ├── SearchBar.svelte         # Global search (Ctrl+K)
    ├── SearchResults.svelte
    └── SearchFilters.svelte
```

## Component Props Convention

```typescript
// Every component exports typed props
interface ButtonProps {
  variant: 'primary' | 'secondary' | 'ghost' | 'danger';
  size: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  loading?: boolean;
  onclick?: () => void;
  children?: Snippet;
}
```

## State Convention

- Components receive data via props (no direct store access in leaf components)
- Store subscriptions happen only at page level or in top-level layout components
- Events bubble up via `dispatch` or callback props
