import type { UserConfig } from 'vite';

import { sveltekit } from '@sveltejs/kit/vite';
import unocss from 'unocss/vite';

const config: UserConfig = {
	plugins: [unocss(), sveltekit()],
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
