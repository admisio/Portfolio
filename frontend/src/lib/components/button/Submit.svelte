<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let enterAllowed: boolean;
	export let value: string;

	const dispatch = createEventDispatcher();

	const handleKeyDown = (e: KeyboardEvent) => {
		if (enterAllowed && e.key === 'Enter') {
			dispatch('click');
		}
	};
</script>

<input on:click type="submit" {value} />

<svelte:window on:keydown={handleKeyDown} />

<style lang="postcss">
	input {
		@apply bg-sspsBlue;
		@apply rounded-lg p-3 text-xl font-semibold text-white transition-colors duration-300;
		@apply w-full;
		--at-apply: 'hover:cursor-pointer hover:bg-sspsBlueDark';
	}
</style>
