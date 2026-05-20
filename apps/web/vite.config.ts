/// <reference types="vitest" />
/// <reference types="vite/client" />

import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import { tanstackRouter } from "@tanstack/router-plugin/vite";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [
    tanstackRouter({
      target: "solid",
      autoCodeSplitting: true,
    }),
    tailwindcss(),
    solid(),
  ],
  server: {
    port: 5173,
  },
});
