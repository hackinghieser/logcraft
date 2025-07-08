// @ts-check
import pluginVue from "eslint-plugin-vue";
import {
  defineConfigWithVueTs,
  vueTsConfigs,
} from "@vue/eslint-config-typescript";
import eslintConfigPrettier from "eslint-config-prettier";
export default defineConfigWithVueTs(
  pluginVue.configs["flat/recommended"],
  vueTsConfigs.recommended,
  eslintConfigPrettier,
  {
    ignores: ["src-tauri/*", "dist/*", "src/vite-env.d.ts"],
  },
);
