# Design System

## Typography

| Element | Font | Size | Weight |
|---------|------|------|--------|
| Heading 1 | Inter | 24px | 700 |
| Heading 2 | Inter | 20px | 600 |
| Heading 3 | Inter | 16px | 600 |
| Body | Inter | 14px | 400 |
| Small | Inter | 12px | 400 |
| Mono (code/IOCs) | JetBrains Mono | 13px | 400 |

## Color Palette

```css
/* Primary */
--color-primary: #2563eb;      /* Blue 600 */
--color-primary-hover: #1d4ed8;
--color-primary-light: #dbeafe;

/* Severity Colors */
--color-critical: #dc2626;     /* Red 600 */
--color-high: #ea580c;         /* Orange 600 */
--color-medium: #ca8a04;       /* Yellow 600 */
--color-low: #16a34a;          /* Green 600 */
--color-info: #6b7280;         /* Gray 500 */

/* Semantic */
--color-success: #16a34a;
--color-warning: #ca8a04;
--color-error: #dc2626;

/* Surface */
--color-bg: #ffffff;
--color-bg-secondary: #f9fafb;
--color-surface: #f3f4f6;
--color-border: #e5e7eb;

/* Text */
--color-text: #111827;
--color-text-secondary: #6b7280;
--color-text-muted: #9ca3af;

/* Dark mode (via .dark class) */
.dark {
  --color-bg: #111827;
  --color-bg-secondary: #1f2937;
  --color-surface: #374151;
  --color-border: #4b5563;
  --color-text: #f9fafb;
  --color-text-secondary: #d1d5db;
}
```

## Spacing

4px grid system: `{space: 4px * multiplier}`
- `sp-1` = 4px
- `sp-2` = 8px
- `sp-3` = 12px
- `sp-4` = 16px
- `sp-6` = 24px
- `sp-8` = 32px
- `sp-12` = 48px
- `sp-16` = 64px

## Component Sizes

| Component | sm | md (default) | lg |
|-----------|----|-------------|-----|
| Button | 32px | 40px | 48px |
| Input | 32px | 40px | 48px |
| Badge | 20px | 24px | 32px |
| Card padding | 16px | 20px | 24px |

## Dark Mode

- Toggle via `data-theme="dark"` on `<html>`
- Persisted in localStorage
- Respects `prefers-color-scheme`

## Accessibility

- All interactive elements keyboard-navigable
- Focus indicators (2px ring, offset 2px)
- Color not sole indicator (icons + labels + color)
- ARIA labels on all icon-only buttons
- Minimum contrast ratio: 4.5:1
