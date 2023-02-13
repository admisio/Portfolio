import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/kit/vite';
import path from 'path';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: [vitePreprocess()],
	kit: {
		adapter: adapter({ out: 'build' }),
		alias: {
			$i18n: path.resolve('./src/translations')
		}
	}
};

export default config;
