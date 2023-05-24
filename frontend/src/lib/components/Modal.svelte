<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
	const close = () => dispatch('close');

	let modal: HTMLElement;

	const handleKeydown = (e: KeyboardEvent) => {
		if (e.key === 'Escape') {
			close();
			return;
		}
	};
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="modalBackground" on:keydown on:click={close} />

<div class="modal" role="dialog" aria-modal="true" bind:this={modal}>
	<slot />
</div>

<style lang="postcss">
	.modalBackground {
		@apply fixed;
		@apply left-0 top-0;
		@apply h-full w-full;
		background: rgba(0, 0, 0, 0.3);

		@apply z-20;
	}

	.modal {
		@apply fixed;
		@apply p-4;
		@apply rounded-xl;
		@apply transform:
		@apply bg-white;

		@apply z-50;

		@apply left-1/2 top-1/2;
		transform: translate(-50%, -50%);
	}
</style>
