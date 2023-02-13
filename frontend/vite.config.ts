import { sveltekit } from '@sveltejs/kit/vite';
import windicss from 'vite-plugin-windicss';

import type { UserConfig } from 'vite';

const config: UserConfig = {
	plugins: [sveltekit(), windicss()],
	server: {
		proxy: {
			'/api': {
				target: 'http://127.0.0.1:8000',
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/api/, '')
			}
		}
	}
};

export default config;
