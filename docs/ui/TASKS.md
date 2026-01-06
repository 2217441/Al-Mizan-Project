# UI Tasks & Outstanding Work

> **Project**: Al-Mizan FYP Showcase Polish  
> **Status**: Complete ✅  
> **Last Updated**: 2026-01-06

---

## Current Sprint

### ✅ Completed This Session (2026-01-06)

| Task | Notes |
|------|-------|
| UI Consolidation | `shared/static/` with symlinks |
| Page renaming | `certainty-engine.html` consistency |
| docs/network.html | Added with theme toggle |
| Navigation sync | 6 links across all pages |
| Theme toggle | All pages working (dark/light) |
| Mobile responsive | CSS breakpoints added |

### ✅ Completed Earlier (2026-01-04)

| Task | Notes |
|------|-------|
| Theme toggle light mode | All pages now working |
| GitHub Pages sync | Rebuilt index.html and style.css |
| Inline style cleanup | Created pages.css, removed from index.html |
| Server restart | SurrealDB + Rust server running |

### ✅ All Verified Working

## Backlog

### P0 - Critical (FYP Demo Blocking)

| Task | Effort | Owner |
|------|--------|-------|
| Fix "Tawhidic" text visibility | 30m | UI Agent |
| Complete theme toggle (all elements) | 1h | UI Agent |
| Test on GitHub Pages | 30m | UI Agent |

### P1 - High (Polish)

| Task | Effort | Owner |
|------|--------|-------|
| Extract inline styles from graph.html | 1h | UI Agent |
| Extract inline styles from governance.html | 1h | UI Agent |
| Extract inline styles from playground.html | 30m | UI Agent |
| Extract inline styles from network.html | 30m | UI Agent |
| Responsive testing (mobile) | 1h | UI Agent |

### P2 - Medium (Nice to Have)

| Task | Effort | Owner |
|------|--------|-------|
| Create static export script for docs/ | 2h | Any |
| Add page transition animations | 1h | UI Agent |
| Improve input focus states | 30m | UI Agent |
| Add loading states/skeletons | 2h | UI Agent |

### P3 - Low (Future)

| Task | Effort | Owner |
|------|--------|-------|
| CSS minification | 1h | DevOps |
| Storybook setup | 4h | Any |
| Visual regression tests | 4h | Any |

---

## Known Bugs

| Bug | Severity | Workaround |
|-----|----------|------------|
| "Tawhidic" text invisible | Medium | Remove `.text-shimmer` class |
| Theme toggle partial | Medium | Manually refresh after toggle |
| Browser caching CSS | Low | Add ?v=N query param |
| Port 3000 conflicts | Low | Kill existing process |

---

## How to Verify Changes

```bash
# 1. Kill existing server
lsof -ti:3000 | xargs kill -9

# 2. Start fresh server
cd /home/a/code/al-mizan-project/almizan-core
cargo run

# 3. Open browser (clear cache)
# Ctrl+Shift+R on http://localhost:3000/landing

# 4. Test theme toggle
# Click ☀️/🌙 button in header
# Verify: body, cards, footer all change
```

---

## Files to Watch

| File | Reason |
|------|--------|
| `static/css/tokens.css` | Design tokens source of truth |
| `static/css/style.css` | Layout and component styles |
| `static/css/pages.css` | Page-specific consolidated styles |
| `templates/base.html` | CSS link tags with versions |
| `docs/style.css` | GitHub Pages styles (must sync) |

---

*Managed by UI Engineering Agent*
