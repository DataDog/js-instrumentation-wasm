import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';
import { InstrumentationTestPlugin } from '@datadog/instrumentation-test-plugin/webpack';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// If we're updating vitest snapshots, do not enable the privacy plugin, so that the
// snapshots reflect the behavior we'd expect *without* the privacy plugin.
const plugins = process.env['UPDATING_SNAPSHOTS'] === 'true'
  ? []
  : [InstrumentationTestPlugin()];

export default {
  entry: "./app/src/main.tsx",

  experiments: {
    outputModule: true,
  },

  externals: {
    react: 'react',
  },

  resolve: {
    extensions: [".tsx", ".ts", ".js", ".jsx"],
    symlinks: false,
  },

  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        loader: 'ts-loader',
        options: {
          configFile: path.resolve(__dirname, './app/tsconfig.app.json'),
        }
      },
      {
        test: /\.css$/,
        use: ["style-loader", "css-loader"],
      },
      {
        test: /\.svg$/,
        use: ['svg-url-loader'],
      },
    ],
  },

  plugins,

  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
    library: {
      type: "module",
    },
  },
};
