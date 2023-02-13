import adapter from '@sveltejs/adapter-node';
import preprocess from 'svelte-preprocess';
import path from 'path';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: [preprocess()],
	kit: {
		adapter: adapter({ out: 'build' }),
		alias: {
			$i18n: path.resolve('./src/translations')
		}
	}
};

export default config;
