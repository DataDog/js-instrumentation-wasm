// vitest.config.js
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    name: 'esbuild-integration',
    environment: 'jsdom',
    globals: true
  },
  resolve: {
    preserveSymlinks: true
  }
});
