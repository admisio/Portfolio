import { defineConfig } from 'unocss';

import presetWind from '@unocss/preset-wind';

import transformerVariantGroup from '@unocss/transformer-variant-group';
import transformerDirectives from '@unocss/transformer-directives';
import transformerCompileClass from '@unocss/transformer-compile-class';

import extractorSvelte from '@unocss/extractor-svelte';

export default defineConfig({
  presets: [presetWind()],
  transformers: [
    transformerVariantGroup(),
    transformerDirectives({
      applyVariable: ['--at-apply', '--uno-apply', '--uno']
    }),
    transformerCompileClass()
  ],
  extractors: [extractorSvelte()],
  content: {
    pipeline: {
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
