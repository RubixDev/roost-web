const path = require('path')
const MonacoWebpackPlugin = require('monaco-editor-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = (_, argv) => ({
	target: 'web',
	mode: argv.mode,
	entry: {
		main: './src/main.ts',
	},
	output: {
		globalObject: 'self',
		filename: '[name].bundle.js',
		path: path.resolve(__dirname, 'dist'),
	},
	resolve: {
		extensions: ['.js', '.ts'],
	},
	module: {
		rules: [
			{
				test: /\.ts$/,
				loader: 'ts-loader',
			},
			{
				test: /\.css$/,
				use: ['style-loader', 'css-loader'],
			},
			{
				test: /\.ttf$/,
				use: ['file-loader'],
			},
		],
	},
	plugins: [
		new MonacoWebpackPlugin({ languages: [] }),
		new WasmPackPlugin({
			crateDirectory: __dirname,
			outDir: path.resolve(__dirname, 'src/pkg'),
			outName: 'roost_web',
			extraArgs: '--target web',
			forceMode: argv.mode,
		}),
	],
})
