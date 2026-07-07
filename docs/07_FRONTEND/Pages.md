# Pages

## Dashboard (`/`)

Shows:
- Open incidents count (by severity)
- Recent incidents (last 24h)
- Technique frequency bar chart
- Team activity feed
- MTTR trend line chart

Data: aggregated from `GET /api/v1/incidents?status=open` + metrics endpoint.

## Incident List (`/incidents`)

**Table columns:** Title, Severity, Status, Techniques, Assignee, Created, Similarity

**Filters:** Status, Severity, Technique, Date range, Tags, Search text

**Features:**
- Sortable columns
- Bulk select → bulk actions (assign, close, delete)
- Inline status change
- Pagination (20 per page)
- Save filter presets

## Incident Detail (`/incidents/[id]`)

Tabbed interface (see UI_Architecture for tabs).
Each tab loads data lazily (only when selected).

Sub-pages:
- `overview/` — summary, techniques, observables, similar incidents
- `timeline/` — interactive timeline
- `evidence/` — file list + upload
- `graph/` — entity relationship graph
- `narrative/` — generated + editable report
- `similar/` — full similarity results with explanations

## New Incident (`/incidents/new`)

Form fields:
- Title (required)
- Description (markdown)
- Severity (dropdown)
- Source (text, e.g., "Sentinel")
- Source ID (optional, external alert ID)
- Observables (dynamic list: type + value pairs)
- Tags (comma-separated)

## Search (`/search`)

- Full-text search across incidents, entities, evidence
- Faceted filters: type (incident, entity, evidence), date range, techniques
- Results grouped by type
- Keyboard shortcut: `Ctrl+K`

## Graph Explorer (`/graph`)

Full-screen graph view (see Graph_Explorer.md for details).

## Settings (`/settings`)

- Profile (name, email, avatar)
- API tokens (create/revoke)
- Notification preferences
- Theme toggle (light/dark)
