import adapter from '@sveltejs/adapter-node';
import path from 'path';
//import { windi } from 'svelte-windicss-preprocess';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: [vitePreprocess()],
	kit: {
		adapter: adapter({ out: 'build' }),
		alias: {
			$i18n: path.resolve('./src/translations')
		}
	}
};

export default config;
