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
	<slot name="header" />
	<hr />
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
		@apply left-1/2 top-1/2;
		@apply w-[calc(100vw - 4em)]
        @apply p-4;
		@apply rounded-md;
		@apply transform:
		translate(-50%, -50%) overflow-auto;
		@apply bg-white;

		@apply z-50;
	}
</style>
