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

        // Ensure container has tablist role
        const tabList = el.querySelector('.tabs') || el;
        if (!tabList.hasAttribute('role')) {
            tabList.setAttribute('role', 'tablist');
        }

        const tabs = Array.from(el.querySelectorAll('.tab'));
        const panels = Array.from(el.querySelectorAll('[data-tab-panel]'));

        tabs.forEach((tab, index) => {
            const panelId = tab.dataset.tab;
            const panel = panels.find(p => p.dataset.tabPanel === panelId);

            // Generate IDs for A11y
            if (!tab.id) tab.id = `tab-${panelId}-${Math.random().toString(36).substr(2, 9)}`;
            if (panel && !panel.id) panel.id = `panel-${panelId}-${Math.random().toString(36).substr(2, 9)}`;

            // Set Tab Roles
            tab.setAttribute('role', 'tab');
            if (panel) tab.setAttribute('aria-controls', panel.id);

            // Set Panel Roles
            if (panel) {
                panel.setAttribute('role', 'tabpanel');
                panel.setAttribute('aria-labelledby', tab.id);
            }

            // Set Initial State
            const isActive = tab.classList.contains('active');
            tab.setAttribute('aria-selected', isActive ? 'true' : 'false');
            tab.setAttribute('tabindex', isActive ? '0' : '-1');

            if (panel) {
                panel.hidden = !isActive;
                panel.style.display = isActive ? 'block' : 'none';
            }

            // Click Handler
            tab.addEventListener('click', () => {
                activateTab(tab, tabs, panels);
            });

            // Keyboard Navigation (Arrow Keys)
            tab.addEventListener('keydown', (e) => {
                let targetIndex = index;
                let handled = false;

                switch (e.key) {
                    case 'ArrowRight':
                        targetIndex = (index + 1) % tabs.length;
                        handled = true;
                        break;
                    case 'ArrowLeft':
                        targetIndex = (index - 1 + tabs.length) % tabs.length;
                        handled = true;
                        break;
                    case 'Home':
                        targetIndex = 0;
                        handled = true;
                        break;
                    case 'End':
                        targetIndex = tabs.length - 1;
                        handled = true;
                        break;
                }

                if (handled) {
                    e.preventDefault();
                    tabs[targetIndex].focus();
                    activateTab(tabs[targetIndex], tabs, panels);
                }
            });
        });
    }

    /**
     * Activate a specific tab
     * @param {Element} selectedTab - Tab to activate
     * @param {Array} tabs - All tabs
     * @param {Array} panels - All panels
     */
    function activateTab(selectedTab, tabs, panels) {
        // Deactivate all
        tabs.forEach(t => {
            t.classList.remove('active');
            t.setAttribute('aria-selected', 'false');
            t.setAttribute('tabindex', '-1');
        });

        panels.forEach(p => {
            p.hidden = true;
            p.style.display = 'none';
        });

        // Activate selected
        selectedTab.classList.add('active');
        selectedTab.setAttribute('aria-selected', 'true');
        selectedTab.setAttribute('tabindex', '0');

        const panelId = selectedTab.dataset.tab;
        const panel = panels.find(p => p.dataset.tabPanel === panelId);
        if (panel) {
            panel.hidden = false;
            panel.style.display = 'block';
        }
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
            toastContainer.setAttribute('role', 'region');
            toastContainer.setAttribute('aria-label', 'Notifications');
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

        // Accessibility attributes
        const isError = type === 'error' || type === 'warning';
        toastEl.setAttribute('role', isError ? 'alert' : 'status');
        toastEl.setAttribute('aria-live', isError ? 'assertive' : 'polite');

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
