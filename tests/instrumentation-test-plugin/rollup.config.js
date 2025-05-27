import commonjs from '@rollup/plugin-commonjs';
import json from '@rollup/plugin-json';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import typescript from '@rollup/plugin-typescript';
import { importAsString } from 'rollup-plugin-string-import';

const entryPoints = [
  'src/index.ts',
  'src/astro.ts',
  'src/rspack.ts',
  'src/vite.ts',
  'src/webpack.ts',
  'src/rollup.ts',
  'src/esbuild.ts',
  'src/farm.ts',

  // TODO: Not currently working with rollup.
  // 'src/nuxt.ts',
];

const externalDependencies = [
  'unplugin',
];

const plugins = () =>
  [
    typescript({
      tsconfig: './tsconfig.json'
    }),
    nodeResolve(),
    commonjs(),
    json(),
    importAsString({
      include: ['**/*.js-txt'],
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
