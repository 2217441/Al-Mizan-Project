# Palette's Journal - Critical Learnings

This journal records critical UX and accessibility learnings for the Almizan project.

## 2025-05-22 - Graph Control Accessibility
**Learning:** Custom graph visualization controls (like zoom/pan) implemented with canvas/SVG overlays often lack semantic meaning for screen readers. Using standard `<button>` elements with `aria-label` and `aria-keyshortcuts` provides a robust bridge without needing complex ARIA grids for the canvas itself.
**Action:** Always ensure overlay controls have explicit `aria-label`s and expose keyboard shortcuts programmatically, even if the main visualization is complex.
