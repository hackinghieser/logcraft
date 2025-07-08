import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import Lara from "@primevue/themes/lara";
import Tooltip from "primevue/tooltip";
import "primeicons/primeicons.css";

const app = createApp(App);
app.use(PrimeVue, {
  theme: {
    preset: Lara,
    options: {
      prefix: "p",
      darkModeSelector: false,
      cssLayer: false,
    },
  },
});

// Theme colors and surface colors will be set dynamically by the theme system

// Add dark mode support
const darkModeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

// Theme state management
let themeMode: 'system' | 'light' | 'dark' = 'system';

function updateTheme(isDark: boolean) {
  if (isDark) {
    // Dark mode surface colors
    document.documentElement.style.setProperty("--p-surface-0", "#0f172a");
    document.documentElement.style.setProperty("--p-surface-50", "#1e293b");
    document.documentElement.style.setProperty("--p-surface-100", "#334155");
    document.documentElement.style.setProperty("--p-surface-200", "#475569");
    document.documentElement.style.setProperty("--p-surface-300", "#64748b");
    document.documentElement.style.setProperty("--p-surface-400", "#94a3b8");
    document.documentElement.style.setProperty("--p-surface-500", "#cbd5e1");
    document.documentElement.style.setProperty("--p-surface-600", "#e2e8f0");
    document.documentElement.style.setProperty("--p-surface-700", "#f1f5f9");
    document.documentElement.style.setProperty("--p-surface-800", "#f8fafc");
    document.documentElement.style.setProperty("--p-surface-900", "#ffffff");
    document.documentElement.style.setProperty("--p-surface-950", "#ffffff");
    
    // Dark mode semantic colors
    document.documentElement.style.setProperty("--p-surface-ground", "#0f172a");
    document.documentElement.style.setProperty("--p-surface-card", "#1e293b");
    document.documentElement.style.setProperty("--p-surface-border", "#334155");
    document.documentElement.style.setProperty("--p-surface-hover", "#334155");
    
    // Dark mode text colors
    document.documentElement.style.setProperty("--p-text-color", "#f1f5f9");
    document.documentElement.style.setProperty("--p-text-muted-color", "#94a3b8");
    document.documentElement.style.setProperty("--p-text-secondary-color", "#cbd5e1");
    
    // Dark mode primary colors (darker oranges for better contrast)
    document.documentElement.style.setProperty("--p-primary-50", "#431407");
    document.documentElement.style.setProperty("--p-primary-100", "#7c2d12");
    document.documentElement.style.setProperty("--p-primary-200", "#9a3412");
    document.documentElement.style.setProperty("--p-primary-300", "#c2410c");
    document.documentElement.style.setProperty("--p-primary-400", "#ea580c");
    document.documentElement.style.setProperty("--p-primary-500", "#f97316");
    document.documentElement.style.setProperty("--p-primary-600", "#fb923c");
    document.documentElement.style.setProperty("--p-primary-700", "#fdba74");
    document.documentElement.style.setProperty("--p-primary-800", "#fed7aa");
    document.documentElement.style.setProperty("--p-primary-900", "#ffedd5");
    document.documentElement.style.setProperty("--p-primary-950", "#fff7ed");
    
    // Add dark theme class to body for CSS targeting
    document.body.classList.add("dark-theme");
    document.body.classList.remove("light-theme");
  } else {
    // Light mode surface colors (restore defaults)
    document.documentElement.style.setProperty("--p-surface-0", "#ffffff");
    document.documentElement.style.setProperty("--p-surface-50", "#f8fafc");
    document.documentElement.style.setProperty("--p-surface-100", "#f1f5f9");
    document.documentElement.style.setProperty("--p-surface-200", "#e2e8f0");
    document.documentElement.style.setProperty("--p-surface-300", "#cbd5e1");
    document.documentElement.style.setProperty("--p-surface-400", "#94a3b8");
    document.documentElement.style.setProperty("--p-surface-500", "#64748b");
    document.documentElement.style.setProperty("--p-surface-600", "#475569");
    document.documentElement.style.setProperty("--p-surface-700", "#334155");
    document.documentElement.style.setProperty("--p-surface-800", "#1e293b");
    document.documentElement.style.setProperty("--p-surface-900", "#0f172a");
    document.documentElement.style.setProperty("--p-surface-950", "#020617");
    
    // Light mode semantic colors
    document.documentElement.style.setProperty("--p-surface-ground", "#f8fafc");
    document.documentElement.style.setProperty("--p-surface-card", "#ffffff");
    document.documentElement.style.setProperty("--p-surface-border", "#e2e8f0");
    document.documentElement.style.setProperty("--p-surface-hover", "#f1f5f9");
    
    // Light mode text colors
    document.documentElement.style.setProperty("--p-text-color", "#1e293b");
    document.documentElement.style.setProperty("--p-text-muted-color", "#64748b");
    document.documentElement.style.setProperty("--p-text-secondary-color", "#475569");
    
    // Light mode primary colors (restore original oranges)
    document.documentElement.style.setProperty("--p-primary-50", "#fff7ed");
    document.documentElement.style.setProperty("--p-primary-100", "#ffedd5");
    document.documentElement.style.setProperty("--p-primary-200", "#fed7aa");
    document.documentElement.style.setProperty("--p-primary-300", "#fdba74");
    document.documentElement.style.setProperty("--p-primary-400", "#fb923c");
    document.documentElement.style.setProperty("--p-primary-500", "#f97316");
    document.documentElement.style.setProperty("--p-primary-600", "#ea580c");
    document.documentElement.style.setProperty("--p-primary-700", "#c2410c");
    document.documentElement.style.setProperty("--p-primary-800", "#9a3412");
    document.documentElement.style.setProperty("--p-primary-900", "#7c2d12");
    document.documentElement.style.setProperty("--p-primary-950", "#431407");
    
    // Add light theme class to body for CSS targeting
    document.body.classList.add("light-theme");
    document.body.classList.remove("dark-theme");
  }
}

// Theme management functions
function applyCurrentTheme() {
  if (themeMode === 'system') {
    updateTheme(darkModeMediaQuery.matches);
  } else {
    updateTheme(themeMode === 'dark');
  }
}

function toggleTheme() {
  themeMode = themeMode === 'light' ? 'dark' : themeMode === 'dark' ? 'system' : 'light';
  applyCurrentTheme();
  return themeMode;
}

// Initial theme setup
applyCurrentTheme();

// Listen for system theme changes (only when in system mode)
darkModeMediaQuery.addEventListener('change', (e) => {
  if (themeMode === 'system') {
    updateTheme(e.matches);
  }
});

// Expose theme functions globally for components to use
(window as any).toggleTheme = toggleTheme;

app.directive("tooltip", Tooltip);
app.mount("#app");
