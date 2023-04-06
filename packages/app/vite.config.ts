import react from '@vitejs/plugin-react';
import path from 'path';
import { defineConfig, type PluginOption } from 'vite';

/**
 * 按需加载 Material UI
 * @returns
 */
const MuiPlugin = (): PluginOption => {
	return {
		name: 'material-ui',
		transform(this, code, id, options?) {
			if (!id.endsWith('.ts') && !id.endsWith('.tsx')) {
				return;
			}

			const MuiRegex = new RegExp(
				/import {(.*?)} from (('@mui\/material')|("@mui\/material"))(;?)/
			);
			const matched = code.match(MuiRegex);
			if (!matched || matched.length < 2) return;

			const imported = matched[1];
			return {
				code: code.replace(
					matched[0],
					imported
						.split(',')
						.map((comp) => comp.trim())
						.filter((comp) => !comp.startsWith('type'))
						.map((comp) => `import ${comp} from "@mui/material/${comp}";\n`)
						.join('')
				),
			};
		},
	};
};

// https://vitejs.dev/config/
export default defineConfig((env) => ({
	plugins: [react(), env.mode === 'production' ? MuiPlugin() : undefined].filter((x) => !!x),
	resolve: {
		alias: {
			'@': path.resolve(__dirname, 'src'),
		},
	},
	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	// prevent vite from obscuring rust errors
	clearScreen: false,
	// tauri expects a fixed port, fail if that port is not available
	server: {
		port: 1420,
		strictPort: true,
	},
	// to make use of `TAURI_DEBUG` and other env variables
	// https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
	envPrefix: ['VITE_', 'TAURI_'],
	build: {
		rollupOptions: {
			input: {
				main: path.resolve(__dirname, 'views', 'main.html'),
				options: path.resolve(__dirname, 'views', 'options.html'),
				splash: path.resolve(__dirname, 'views', 'splash.html'),
			},
		},
		// Tauri supports es2021
		target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
		// don't minify for debug builds
		minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
		// produce sourcemaps for debug builds
		sourcemap: !!process.env.TAURI_DEBUG,
	},
	css: {
		//* css模块化
		modules: {
			// css模块化 文件以.module.[css|less|scss]结尾
			generateScopedName: '[name]__[local]___[hash:base64:5]',
			hashPrefix: 'prefix',
		},
		//* 预编译支持less
		preprocessorOptions: {
			less: {
				// 支持内联 JavaScript
				javascriptEnabled: true,
			},
		},
	},
}));
