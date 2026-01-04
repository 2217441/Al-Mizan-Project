# UI Runbooks

> Quick reference procedures for common UI development tasks

---

## Theme Toggle Debug

### Symptom: Theme toggle doesn't change all elements

```bash
# 1. Check CSS files are loading
# Open DevTools > Network > Filter: css
# Should see: tokens.css, effects.css, components.css, style.css, pages.css

# 2. Check theme attribute
# DevTools > Console
document.documentElement.getAttribute('data-theme')
# Should return "dark" or "light"

# 3. Check CSS variable values
getComputedStyle(document.documentElement).getPropertyValue('--color-void')
# Dark: #050510, Light: #f5f3ef
```

### Fix: Add missing light mode override

```css
/* In style.css or pages.css */
[data-theme="light"] .my-element {
    background: rgba(255, 255, 255, 0.95);
    border-color: rgba(0, 0, 0, 0.1);
    color: var(--text-primary);
}
```

---

## Clear Browser Cache

### Chrome/Edge

```
Ctrl+Shift+Delete
OR
Ctrl+Shift+R (hard refresh)
```

### Programmatic (via query string)

```html
<link rel="stylesheet" href="/static/css/style.css?v=7">
```

---

## Port Conflict Resolution

### Symptom: "Address already in use (os error 98)"

```bash
# Find process on port 3000
lsof -ti:3000

# Kill it
lsof -ti:3000 | xargs kill -9

# Restart server
cd /home/a/code/al-mizan-project/almizan-core
cargo run
```

---

## Add New Page-Specific Styles

### Step 1: Define styles in `pages.css`

```css
/* ============================================
   NEW PAGE (newpage.html)
   ============================================ */

.newpage-wrapper {
    min-height: calc(100vh - 70px);
    padding: var(--space-8);
}

.newpage-header {
    /* Use design tokens */
    font-family: var(--font-heading);
    color: var(--color-gold);
}
```

### Step 2: Use in template

```html
{% block content %}
<!-- Styles in pages.css -->
<div class="newpage-wrapper noise-overlay">
    <h1 class="newpage-header">Title</h1>
</div>
{% endblock %}
```

---

## Sync GitHub Pages

### When to Sync

- After major UI changes
- Before FYP presentation
- After design token updates

### Manual Sync Process

```bash
# 1. Update docs/style.css with token values
# Ensure variables match tokens.css

# 2. Update docs/index.html structure
# Match landing.html layout

# 3. Test locally
python -m http.server 8080 --directory docs/
# Open http://localhost:8080

# 4. Commit and push
git add docs/
git commit -m "Sync GitHub Pages with local design"
git push
```

---

## Common CSS Patterns

### Glass Panel

```css
.my-glass {
    background: var(--color-panel);
    backdrop-filter: blur(12px) saturate(180%);
    border: var(--border-glass);
    border-radius: var(--radius-xl);
    box-shadow: var(--shadow-lg);
}
```

### Golden Glow Button

```css
.my-btn {
    background: var(--color-gold);
    color: var(--text-inverse);
    border: none;
    padding: var(--space-3) var(--space-6);
    border-radius: var(--radius-lg);
    box-shadow: 0 0 20px rgba(212, 175, 55, 0.3);
    transition: all 0.3s ease;
}

.my-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 0 30px rgba(212, 175, 55, 0.5);
}
```

### Animated Text Reveal

```html
<span class="text-reveal" style="animation-delay: 0.1s;">First</span>
<span class="text-reveal" style="animation-delay: 0.2s;">Second</span>
```

### Status Indicator

```html
<div class="badge">
    <span class="badge__dot"></span>
    <span>Online</span>
</div>
```

---

## Debug Invisible Text

### Symptom: Text exists but invisible

```javascript
// Find element
const el = document.querySelector('.text-shimmer');

// Check computed styles
const style = getComputedStyle(el);
console.log({
    color: style.color,
    textFillColor: style.webkitTextFillColor,
    backgroundImage: style.backgroundImage,
    backgroundClip: style.backgroundClip
});
```

### Common Causes

1. `-webkit-text-fill-color: transparent` without `background-image`
2. CSS variable undefined (returns empty string)
3. `opacity: 0` or `visibility: hidden`

### Quick Fix

```css
.text-shimmer {
    /* Add fallback */
    color: var(--color-gold);
}
```

---

*Quick reference for UI Engineering tasks*
