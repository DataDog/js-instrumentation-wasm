// vitest.config.js
import { defineConfig } from 'vitest/config';

export default defineConfig({
  resolve: {
    preserveSymlinks: true,
  },
  test: {
    name: "webpack-integration",
    environment: "jsdom",
  },
});
