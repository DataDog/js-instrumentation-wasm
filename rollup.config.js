import commonjs from '@rollup/plugin-commonjs';
import json from '@rollup/plugin-json';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import typescript from '@rollup/plugin-typescript';
import { wasm } from '@rollup/plugin-wasm';

const entryPoints = ['src/index.ts'];

const externalDependencies = [];

const plugins = () =>
  [
    typescript({
      tsconfig: './tsconfig.json'
    }),
    nodeResolve(),
    commonjs(),
    json(),
    wasm({
      targetEnv: 'auto-inline'
    }),
  ];

export default [
  {
    input: entryPoints,
    plugins: plugins(),
    external: externalDependencies,
    output: {
      dir: 'dist/esm',
      chunkFileNames: '[name]-[hash].mjs',
      entryFileNames: '[name].mjs',
      exports: 'named',
      format: 'es',
    },
  },
  {
    input: entryPoints,
    plugins: plugins(),
    external: externalDependencies,
    output: {
      dir: 'dist/cjs',
      chunkFileNames: '[name]-[hash].cjs',
      entryFileNames: '[name].cjs',
      exports: 'named',
      format: 'cjs',
    },
  }
];
