import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import Aura from "@primevue/themes/aura";
import Tooltip from "primevue/tooltip";
import "primeicons/primeicons.css";

const app = createApp(App);
app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      prefix: "p",
      darkModeSelector: "system",
      cssLayer: false
    }
  }
});
app.directive('tooltip', Tooltip);
app.mount("#app");
