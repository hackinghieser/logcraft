import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import { definePreset } from "@primevue/themes";
import Aura from "@primevue/themes/aura";
import Tooltip from "primevue/tooltip";
import "primeicons/primeicons.css";

const TimberPreset = definePreset(Aura, {
  semantic: {
    primary: {
      50: "{orange.50}",
      100: "{orange.100}",
      200: "{orange.200}",
      300: "{orange.300}",
      400: "{orange.400}",
      500: "{orange.500}",
      600: "{orange.600}",
      700: "{orange.700}",
      800: "{orange.800}",
      900: "{orange.900}",
      950: "{orange.950}",
    },
    surface: {
      0: "{slate.0}",
      50: "{slate.50}",
      100: "{slate.100}",
      200: "{slate.200}",
      300: "{slate.300}",
      400: "{slate.400}",
      500: "{slate.500}",
      600: "{slate.600}",
      700: "{slate.700}",
      800: "{slate.800}",
      900: "{slate.900}",
      950: "{slate.950}",
    },
  },
  light: {
    primary: {
      50: "{orange.50}",
      100: "{orange.100}",
      200: "{orange.200}",
      300: "{orange.300}",
      400: "{orange.400}",
      500: "{orange.500}",
      600: "{orange.600}",
      700: "{orange.700}",
      800: "{orange.800}",
      900: "{orange.900}",
      950: "{orange.950}",
    },
    surface: {
      0: "{slate.0}",
      50: "{slate.50}",
      100: "{slate.100}",
      200: "{slate.200}",
      300: "{slate.300}",
      400: "{slate.400}",
      500: "{slate.500}",
      600: "{slate.600}",
      700: "{slate.700}",
      800: "{slate.800}",
      900: "{slate.900}",
      950: "{slate.950}",
    },
  },
  dark: {
    primary: {
      50: "{orange.950}",
      100: "{orange.900}",
      200: "{orange.800}",
      300: "{orange.700}",
      400: "{orange.600}",
      500: "{orange.500}",
      600: "{orange.400}",
      700: "{orange.300}",
      800: "{orange.200}",
      900: "{orange.100}",
      950: "{orange.50}",
    },
    surface: {
      0: "{slate.950}",
      50: "{slate.900}",
      100: "{slate.800}",
      200: "{slate.700}",
      300: "{slate.600}",
      400: "{slate.500}",
      500: "{slate.400}",
      600: "{slate.300}",
      700: "{slate.200}",
      800: "{slate.100}",
      900: "{slate.50}",
      950: "{slate.0}",
    },
  },
});

const app = createApp(App);
app.use(PrimeVue, {
  theme: {
    preset: TimberPreset,
    options: {
      prefix: "p",
      darkModeSelector: ".dark-theme",
      cssLayer: false,
    },
  },
});

// Add dark mode support
const darkModeMediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

// Theme state management
let themeMode: "system" | "light" | "dark" = "system";

function updateTheme(isDark: boolean) {
  if (isDark) {
    document.documentElement.classList.add("dark-theme");
    document.documentElement.classList.remove("light-theme");
  } else {
    document.documentElement.classList.add("light-theme");
    document.documentElement.classList.remove("dark-theme");
  }

  // Force a repaint to ensure theme changes are applied
  document.body.style.display = "none";
  document.body.offsetHeight; // Trigger reflow
  document.body.style.display = "";
}

// Theme management functions
function applyCurrentTheme() {
  if (themeMode === "system") {
    updateTheme(darkModeMediaQuery.matches);
  } else {
    updateTheme(themeMode === "dark");
  }
}

function toggleTheme() {
  // Simple two-state toggle: light <-> dark
  const currentIsDark =
    themeMode === "dark" ||
    (themeMode === "system" && darkModeMediaQuery.matches);

  themeMode = currentIsDark ? "light" : "dark";
  applyCurrentTheme();
  return themeMode;
}

// Initial theme setup
applyCurrentTheme();

// Listen for system theme changes (only when in system mode)
darkModeMediaQuery.addEventListener("change", (e) => {
  if (themeMode === "system") {
    updateTheme(e.matches);
  }
});

// Expose theme functions globally for components to use
(window as any).toggleTheme = toggleTheme;

app.directive("tooltip", Tooltip);
app.mount("#app");
