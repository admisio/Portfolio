import { defineConfig } from 'unocss';

import presetWind from '@unocss/preset-wind';

import transformerVariantGroup from '@unocss/transformer-variant-group';
import transformerDirectives from '@unocss/transformer-directives';
import transformerCompileClass from '@unocss/transformer-compile-class';

import extractorSvelte from '@unocss/extractor-svelte';

export default defineConfig({
	autocomplete: true,
	presets: [presetWind()],
	transformers: [transformerVariantGroup(), transformerDirectives(), transformerCompileClass()],
	extractors: [extractorSvelte()],
	content: {
		pipeline: {
			include: ['src/**/*.{svelte,html}'],
			exclude: [
				'node_modules',
				'.git',
				'dist',
				'build',
				'public',
				'src/translations',
				'src/lib/assets'
			]
		}
	},
	theme: {
		colors: {
			sspsBlue: '#406280',
			sspsBlueDark: '#243a55',
			sspsGray: '#e6e6e6'
		}
	}
});
