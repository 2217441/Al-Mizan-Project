/**
 * Al-Mizan Component Library - JavaScript Utilities
 * Interactive component behaviors
 */

(function() {
    'use strict';

    // =========================================
    // TABS
    // =========================================
    
    /**
     * Initialize tab navigation
     * @param {string|Element} container - Container selector or element
     */
    function initTabs(container) {
        const el = typeof container === 'string' ? document.querySelector(container) : container;
        if (!el) return;

        const tabs = el.querySelectorAll('.tab');
        const panels = el.querySelectorAll('[data-tab-panel]');

        tabs.forEach(tab => {
            tab.addEventListener('click', () => {
                // Remove active from all tabs
                tabs.forEach(t => t.classList.remove('active'));
                // Add active to clicked tab
                tab.classList.add('active');

                // Show corresponding panel
                const panelId = tab.dataset.tab;
                panels.forEach(panel => {
                    panel.style.display = panel.dataset.tabPanel === panelId ? 'block' : 'none';
                });
            });
        });
    }

    // =========================================
    // GAUGE
    // =========================================
    
    /**
     * Animate a gauge to a specific value
     * @param {string|Element} element - Gauge element or selector
     * @param {number} value - Value between 0 and 1
     * @param {string} color - Optional stroke color
     */
    function initGauge(element, value, color) {
        const el = typeof element === 'string' ? document.querySelector(element) : element;
        if (!el) return;

        const progress = el.querySelector('.gauge__progress');
        const text = el.querySelector('.gauge__text');
        
        if (!progress) return;

        // Calculate stroke offset (283 is circumference of r=45 circle)
        const offset = 283 - (value * 283);
        
        // Apply color if provided
        if (color) {
            progress.style.stroke = color;
        }

        // Animate after a small delay for visual effect
        requestAnimationFrame(() => {
            progress.style.strokeDashoffset = offset;
            if (text) {
                const percentage = Math.round(value * 100);
                text.textContent = percentage + '%';

                // Update ARIA values for accessibility
                el.setAttribute('aria-valuenow', percentage);
                el.setAttribute('aria-valuemin', '0');
                el.setAttribute('aria-valuemax', '100');
                if (!el.hasAttribute('role')) {
                    el.setAttribute('role', 'progressbar');
                }
            }
        });
    }

    // =========================================
    // DETAIL PANEL
    // =========================================
    
    /**
     * Initialize detail panel toggle
     * @param {string} panelSelector - Panel element selector
     * @param {object} options - Configuration options
     */
    function initDetailPanel(panelSelector, options = {}) {
        const panel = document.querySelector(panelSelector);
        if (!panel) return;

        const closeBtn = panel.querySelector('.detail-panel__close');
        if (closeBtn) {
            closeBtn.addEventListener('click', () => closeDetailPanel(panelSelector));
        }

        // Close on Escape key
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && panel.classList.contains('active')) {
                closeDetailPanel(panelSelector);
            }
        });

        // Optional: close on backdrop click
        if (options.closeOnBackdrop) {
            document.addEventListener('click', (e) => {
                if (panel.classList.contains('active') && !panel.contains(e.target)) {
                    closeDetailPanel(panelSelector);
                }
            });
        }
    }

    function openDetailPanel(panelSelector, data = {}) {
        const panel = document.querySelector(panelSelector);
        if (!panel) return;

        // Populate data if provided
        Object.entries(data).forEach(([key, value]) => {
            const el = panel.querySelector(`[data-field="${key}"]`);
            if (el) el.textContent = value;
        });

        panel.classList.add('active');
    }

    function closeDetailPanel(panelSelector) {
        const panel = document.querySelector(panelSelector);
        if (panel) panel.classList.remove('active');
    }

    // =========================================
    // TOAST NOTIFICATIONS
    // =========================================
    
    let toastContainer = null;

    /**
     * Show a toast notification
     * @param {string} message - Message to display
     * @param {string} type - 'success' | 'error' | 'warning' | 'info'
     * @param {number} duration - Duration in ms (default 3000)
     */
    function toast(message, type = 'info', duration = 3000) {
        // Create container if it doesn't exist
        if (!toastContainer) {
            toastContainer = document.createElement('div');
            toastContainer.className = 'toast-container';
            document.body.appendChild(toastContainer);
        }

        // Create toast element
        const toastEl = document.createElement('div');
        toastEl.className = `toast toast--${type}`;

        // Accessibility: Set role and aria-live based on type
        if (type === 'error') {
            toastEl.setAttribute('role', 'alert');
            toastEl.setAttribute('aria-live', 'assertive');
        } else {
            toastEl.setAttribute('role', 'status');
            toastEl.setAttribute('aria-live', 'polite');
        }

        toastEl.textContent = message;

        // Add to container
        toastContainer.appendChild(toastEl);

        // Auto-remove after duration
        setTimeout(() => {
            toastEl.classList.add('removing');
            setTimeout(() => {
                toastEl.remove();
            }, 300);
        }, duration);

        return toastEl;
    }

    // =========================================
    // COPY TO CLIPBOARD
    // =========================================
    
    /**
     * Copy text to clipboard with toast feedback
     * @param {string} text - Text to copy
     * @param {string} successMessage - Optional success message
     */
    async function copyToClipboard(text, successMessage = 'Copied to clipboard!') {
        try {
            await navigator.clipboard.writeText(text);
            toast(successMessage, 'success');
            return true;
        } catch (err) {
            toast('Failed to copy', 'error');
            return false;
        }
    }

    // =========================================
    // FORM VALIDATION HELPERS
    // =========================================
    
    /**
     * Add loading state to a button
     * @param {Element} button - Button element
     * @param {boolean} loading - Loading state
     */
    function setButtonLoading(button, loading) {
        if (loading) {
            button.dataset.originalText = button.textContent;
            button.disabled = true;
            button.style.opacity = '0.5';
            button.textContent = 'Loading...';
        } else {
            button.disabled = false;
            button.style.opacity = '1';
            button.textContent = button.dataset.originalText || button.textContent;
        }
    }

    // =========================================
    // ANIMATIONS
    // =========================================
    
    /**
     * Stagger animation for list items
     * @param {string} containerSelector - Container with items to animate
     * @param {string} itemSelector - Selector for items within container
     * @param {number} delay - Delay between items in ms
     */
    function staggerAnimation(containerSelector, itemSelector = '.stagger-item', delay = 50) {
        const container = document.querySelector(containerSelector);
        if (!container) return;

        const items = container.querySelectorAll(itemSelector);
        items.forEach((item, index) => {
            item.style.animationDelay = `${index * delay}ms`;
        });
    }

    // =========================================
    // EXPORT TO GLOBAL
    // =========================================
    
    window.AlMizan = window.AlMizan || {};
    window.AlMizan.components = {
        initTabs,
        initGauge,
        initDetailPanel,
        openDetailPanel,
        closeDetailPanel,
        toast,
        copyToClipboard,
        setButtonLoading,
        staggerAnimation
    };

    // Auto-initialize on DOMContentLoaded
    document.addEventListener('DOMContentLoaded', () => {
        // Auto-init tabs
        document.querySelectorAll('[data-tabs]').forEach(el => initTabs(el));
        
        // Auto-init detail panels
        document.querySelectorAll('.detail-panel').forEach(el => {
            initDetailPanel(`#${el.id}`, { closeOnBackdrop: true });
        });
    });

})();
