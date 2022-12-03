<script lang="ts">
	import Circles from '$lib/components/icons/Circles.svelte';

	export let title: string;
</script>

<div class="card infoCard relative">
	<span class="absolute -left-8 -top-8">
		<Circles />
	</span>
	<div class="mt-20 flex flex-col">
		<h3>{title}</h3>
		<slot />
	</div>
</div>

<style>
	.card {
		@apply m-3;
		@apply h-full;
		@apply flex flex-col justify-between;

		@apply bg-[#f8fbfc];
		@apply rounded-3xl;
		@apply px-7 py-10;

		@apply transition-all duration-300;
	}
	.card:hover {
		@apply shadow-2xl;
		@apply m-0;
	}
	.card h3 {
		@apply text-sspsBlue text-4xl md:text-2xl xl:text-4xl font-semibold;
	}
</style>
