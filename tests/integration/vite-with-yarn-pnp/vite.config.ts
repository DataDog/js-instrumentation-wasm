import process from 'node:process';
import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react'
import { InstrumentationTestPlugin } from '@datadog/instrumentation-test-plugin/vite';

const basePlugins = [
  react()
];

// If we're updating vitest snapshots, only enable the base plugins, so that the snapshots
// reflect the behavior we'd expect *without* the privacy plugin. Otherwise, enable the
// privacy plugin as well, so that we build and run tests with it enabled.
const plugins = process.env['UPDATING_SNAPSHOTS'] === 'true'
  ? basePlugins
  : [...basePlugins, InstrumentationTestPlugin()];

// https://vite.dev/config/
export default defineConfig({
  plugins,
  test: {
    name: 'integration-vite-with-yarn-pnp',
    environment: 'jsdom',
  },
  resolve: {
    preserveSymlinks: true,
  },
  build: {
    rollupOptions: {
      input: {
        main: './app/src/main.tsx',
      },
    },
  },
})
