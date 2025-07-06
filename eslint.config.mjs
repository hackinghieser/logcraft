// @ts-check
import pluginVue from "eslint-plugin-vue";
import {
  defineConfigWithVueTs,
  vueTsConfigs,
} from "@vue/eslint-config-typescript";
export default defineConfigWithVueTs(
  pluginVue.configs["flat/recommended"],
  vueTsConfigs.recommended,
  {
    ignores: ["src-tauri/*", "dist/*", "src/vite-env.d.ts"],
  },
);
