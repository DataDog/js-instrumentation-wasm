import { defineConfig } from 'vite';
import commonjs from "vite-plugin-commonjs";
import vitePluginRequire from "vite-plugin-require";

export default defineConfig({
  plugins: [commonjs(), vitePluginRequire.default()],
  test: {
    name: 'unit',
  }
});
