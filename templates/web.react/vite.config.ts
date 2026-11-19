import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";
// {if:web.pwa}
import { serviceWorker } from "./vite-sw.ts";
// {endif:web.pwa}

export default defineConfig({
  // {ifnot:web.pwa}
  plugins: [react()],
  // {endif:web.pwa}
  // {if:web.pwa}
  plugins: [react(), serviceWorker()],
  // {endif:web.pwa}
  test: {
    environment: "jsdom",
    setupFiles: ["./src/setup-tests.ts"],
  },
});
