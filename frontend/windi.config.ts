import { defineConfig } from 'windicss/helpers';

export default defineConfig({
	extract: {
		include: ['src/**/*.{svelte,html,js,ts}'],
		exclude: [
			'node_modules',
			'.git',
			'dist',
			'build',
			'public',
			'src/translations',
			'src/lib/assets'
		]
	},
	theme: {
		extend: {
			colors: {
				sspsBlue: '#406280',
				sspsBlueDark: '#243a55',
				sspsGray: '#e6e6e6'
			}
		}
	}
});
