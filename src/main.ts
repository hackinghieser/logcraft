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
      darkModeSelector: "system",
      cssLayer: false
    }
  }
});

// Customize theme colors
document.documentElement.style.setProperty('--p-primary-50', '#fff7ed');
document.documentElement.style.setProperty('--p-primary-100', '#ffedd5');
document.documentElement.style.setProperty('--p-primary-200', '#fed7aa');
document.documentElement.style.setProperty('--p-primary-300', '#fdba74');
document.documentElement.style.setProperty('--p-primary-400', '#fb923c');
document.documentElement.style.setProperty('--p-primary-500', '#f97316'); // Orange-500
document.documentElement.style.setProperty('--p-primary-600', '#ea580c');
document.documentElement.style.setProperty('--p-primary-700', '#c2410c');
document.documentElement.style.setProperty('--p-primary-800', '#9a3412');
document.documentElement.style.setProperty('--p-primary-900', '#7c2d12');
document.documentElement.style.setProperty('--p-primary-950', '#431407');

// Set slate surface colors
document.documentElement.style.setProperty('--p-surface-0', '#ffffff');
document.documentElement.style.setProperty('--p-surface-50', '#f8fafc');
document.documentElement.style.setProperty('--p-surface-100', '#f1f5f9');
document.documentElement.style.setProperty('--p-surface-200', '#e2e8f0');
document.documentElement.style.setProperty('--p-surface-300', '#cbd5e1');
document.documentElement.style.setProperty('--p-surface-400', '#94a3b8');
document.documentElement.style.setProperty('--p-surface-500', '#64748b');
document.documentElement.style.setProperty('--p-surface-600', '#475569');
document.documentElement.style.setProperty('--p-surface-700', '#334155');
document.documentElement.style.setProperty('--p-surface-800', '#1e293b');
document.documentElement.style.setProperty('--p-surface-900', '#0f172a');
document.documentElement.style.setProperty('--p-surface-950', '#020617');

// Set surface semantic colors
document.documentElement.style.setProperty('--p-surface-ground', '#f8fafc');
document.documentElement.style.setProperty('--p-surface-card', '#ffffff');
document.documentElement.style.setProperty('--p-surface-border', '#e2e8f0');

app.directive('tooltip', Tooltip);
app.mount("#app");
