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

<style>
	.modalBackground {
		@apply fixed;
		@apply top-0 left-0;
		@apply h-full w-full;
		background: rgba(0, 0, 0, 0.3);

		@apply z-20;
	}

	.modal {
		@apply absolute;
		@apply p-4;
		@apply rounded-md;
		@apply transform:
		@apply bg-white;

		@apply z-50;

		@apply top-1/2 left-1/2;
		transform: translate(-50%, -50%);
	}
</style>
