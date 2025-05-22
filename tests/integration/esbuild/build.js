const process = require('node:process');
const esbuild = require('esbuild');
const { InstrumentationTestPlugin } = require('@datadog/instrumentation-test-plugin/esbuild')

// If we're updating vitest snapshots, do not enable the privacy plugin, so that the
// snapshots reflect the behavior we'd expect *without* the privacy plugin.
const plugins = process.env['UPDATING_SNAPSHOTS'] === 'true'
  ? []
  : [InstrumentationTestPlugin()];

/** @type {import('esbuild').BuildOptions} */
const config = {
  entryPoints: ["./app/src/main.tsx"],
  bundle: true,
  outfile: "dist/index.js",
  platform: "browser",
  target: ["esnext"],
  loader: {
    ".tsx": "tsx",
    ".ts": "tsx",
    ".css": "css",
    ".svg": "dataurl",
  },
  define: {
    "process.env.NODE_ENV": JSON.stringify("production"),
  },
  sourcemap: false,
  minify: true,
  plugins,
  preserveSymlinks: true,
  external: ["react", "react-dom"],
  format: "esm",
  jsx: "automatic",
};

esbuild.build(config).then(() => {
  console.log('Build complete');
});
