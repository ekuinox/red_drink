const HtmlWebpackPlugin = require('html-webpack-plugin');

const env = process.env.WEBPACK_ENV || 'production';

module.exports = {
    mode: env,
	devtool: env == 'development' ? "inline-source-map" : undefined,
	entry: './src/Index.tsx',
	output: {
		path: `${__dirname}/dist`,
		filename: 'bundle.js',
	},
	resolve: {
		extensions: ['.ts', '.tsx', '.js']
	},
	module: {
		rules: [{
			test: /\.tsx?$/,
			exclude: /node_modules/,
			loader: 'ts-loader'
		}]
	},
	plugins: [
		new HtmlWebpackPlugin({
			title: "red drink",
			inject: false,
			template: "./src/index.html"
		})
	]
};
