import { Configuration } from 'webpack';
import HtmlWebpackPlugin from 'html-webpack-plugin';
import { argv } from 'yargs';
import { resolve } from 'path';

type Mode = 'development' | 'production' | 'none';

const getMode = (): Mode => {
  if (
    typeof argv['mode'] === 'string' &&
    ['development', 'production', 'none'].includes(argv['mode'])
  )
    return argv['mode'] as Mode;
  return 'production';
};

const mode = getMode();

const conf: Configuration = {
  mode,
  devtool: mode === 'development' ? 'inline-source-map' : undefined,
  entry: './src/Index.tsx',
  output: {
    path: resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  resolve: {
    modules: [resolve(__dirname, 'src'), 'node_modules'],
    extensions: ['.ts', '.tsx', '.js'],
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        loader: 'babel-loader',
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: 'red drink',
      inject: false,
      template: './src/index.html',
    }),
  ],
};

export default conf;
