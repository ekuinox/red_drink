import * as HtmlWebpackPlugin from 'html-webpack-plugin';
import { Configuration } from 'webpack';

const isMode = (mode?: string): mode is 'development' | 'production' | 'none' =>
  ['development', 'production', 'none'].includes(mode ?? '');

const env = isMode(process.env.WEBPACK_ENV)
  ? process.env.WEBPACK_ENV
  : 'production';

const conf: Configuration = {
  mode: env,
  devtool: env === 'development' ? 'inline-source-map' : undefined,
  entry: './src/Index.tsx',
  output: {
    path: `${__dirname}/dist`,
    filename: 'bundle.js',
  },
  resolve: {
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
