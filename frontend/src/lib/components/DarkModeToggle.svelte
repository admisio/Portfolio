<script lang="ts">
	export let backgroundColor: 'light' | 'dark' = 'light';
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	class:blue={backgroundColor === 'light'}
	class:white={backgroundColor === 'dark'}
	viewBox="0 0 24 24"
	stroke-width="2"
	stroke-linecap="round"
	stroke-linejoin="round"
>
	<circle cx="12" cy="12" r="5" />
	<g>
		<line x1="12" y1="1" x2="12" y2="3" />
		<line x1="12" y1="21" x2="12" y2="23" />
		<line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
		<line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
		<line x1="1" y1="12" x2="3" y2="12" />
		<line x1="21" y1="12" x2="23" y2="12" />
		<line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
		<line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
	</g>
</svg>

<style lang="postcss">
	svg {
		@apply h-12 w-12;
	}
	.blue {
		@apply stroke-sspsBlue fill-sspsBlue;
		@apply hover:fill-sspsBlueDark hover:stroke-sspsBlueDark transition-colors duration-300;
	}
	.white {
		@apply fill-white stroke-white;
	}
</style>
