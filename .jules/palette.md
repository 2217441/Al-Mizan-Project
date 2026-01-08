## 2024-05-23 - Gauge Component Accessibility
**Learning:** Custom visual components like SVG gauges are completely invisible to screen readers without ARIA roles.
**Action:** Always add `role="progressbar"` and keep `aria-valuenow` synced via JS when animating custom progress indicators.
