<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let value: string;

	export let id: number | undefined;

	let isPrepared = false;

	const buttonLogic = () => {
		if (isPrepared) {
			dispatch('delete', {
				id: id
			});
		} else {
			dispatch('prepared', {
				id: id
			});
			isPrepared = true;
			setTimeout(() => {
				isPrepared = false;
			}, 3000);
		}
	};
</script>

<button on:click={buttonLogic} class="animate-bounce" class:isPrepared>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		class="mr-2 h-5 w-5"
		fill="none"
		viewBox="0 0 24 24"
		stroke="currentColor"
	>
		<path
			stroke-linecap="round"
			stroke-linejoin="round"
			stroke-width="2"
			d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
		/>
	</svg>

	{value}
</button>

<style lang="postcss">
	button {
		@apply inline-flex items-center;
		@apply bg-red-700;
		@apply rounded-lg p-3 font-semibold 
        text-white transition-colors duration-300;

		animation: none !important;
	}
	button:hover {
		@apply bg-red-800;
	}

	.isPrepared {
		@apply bg-red-800;
		animation: bounce 1s infinite !important;
	}
</style>
