# Palette's Journal - Critical Learnings

This journal records critical UX and accessibility learnings for the Almizan project.

## 2025-05-22 - Graph Control Accessibility
**Learning:** Custom graph visualization controls (like zoom/pan) implemented with canvas/SVG overlays often lack semantic meaning for screen readers. Using standard `<button>` elements with `aria-label` and `aria-keyshortcuts` provides a robust bridge without needing complex ARIA grids for the canvas itself.
**Action:** Always ensure overlay controls have explicit `aria-label`s and expose keyboard shortcuts programmatically, even if the main visualization is complex.

## 2025-10-24 - Canvas-DOM Focus Handoff
**Learning:** Interaction with canvas-based elements (like graph nodes) breaks the natural DOM focus flow. Explicitly moving focus to a DOM element (like a panel close button) is required when a canvas interaction triggers a UI overlay, otherwise keyboard users are left stranded.
**Action:** Implement a `shouldFocus` parameter in UI trigger functions to handle focus movement conditionally (e.g., focus on click, but not on search-triggered updates).

## 2025-10-25 - Synchronized Accessible Tooltips
**Learning:** Icon-only buttons often have `aria-label` for screen readers but lack tooltips for mouse users, leaving them guessing. Dynamically synchronizing the `title` attribute with `aria-label` ensures consistent terminology for all users without duplicate maintenance.
**Action:** When updating accessible labels in JS, also update the `title` attribute for icon-only controls.

## 2025-05-23 - Keyboard Trap in Inputs
**Learning:** Global keyboard shortcuts often exclude inputs to avoid conflict with typing, but this inadvertently disables standard keys like `Escape` for clearing/blurring.
**Action:** When filtering global keydown events, explicitly allow `Escape` (and potentially `Enter`) to pass through the `input` filter.
