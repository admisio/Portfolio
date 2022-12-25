<script lang="ts">
	import { tippy } from 'svelte-tippy';
	import 'tippy.js/dist/tippy.css';
	import { createEventDispatcher } from 'svelte';
	import { submissionProgress, UploadStatus } from '$lib/stores/portfolio';
	import Document from '../icons/Document.svelte';
	import Download from '../icons/Download.svelte';

	export let showDetails: boolean;

	const dispatch = createEventDispatcher();

	const showInfo = () => {
		dispatch('showInfo');
	};

	const download = () => {
		dispatch('download');
	};
</script>

<span
	on:click={(_) => showInfo()}
	on:keydown={(_) => showInfo()}
	use:tippy={{
		content: (showDetails ? 'Skrýt' : 'Zobrazit') + ' osobní údaje',
		placement: 'top',
		showOnCreate: false,
		delay: 0
	}}
	class="icon"
	class:showDetails
>
	<Document />
</span>

{#if $submissionProgress.status === UploadStatus.Submitted}
	<span
		on:click={(_) => download()}
		on:keydown={(_) => download()}
		use:tippy={{
			content: 'Stáhnout portfolio',
			placement: 'top',
			showOnCreate: false,
			delay: 0
		}}
	>
		<svg
			class="icon"
			fill="none"
			stroke="currentColor"
			viewBox="0 0 24 24"
			xmlns="http://www.w3.org/2000/svg"
			><path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
			/></svg
		>
	</span>
{:else}
	<span
		use:tippy={{
			content: 'Nelze stáhnout, portfolio nebylo odevzdáno',
			placement: 'top',
			showOnCreate: false,
			delay: 0
		}}
		class="icon disabledIcon"
	>
		<Download />
	</span>
{/if}

<style lang="postcss">
	.icon {
		@apply text-sspsBlueDark h-10 w-10 transition-colors duration-300 hover:cursor-pointer;
		@apply hover:text-sspsBlue;
	}
	.showDetails {
		@apply text-sspsBlue;
	}
	.disabledIcon {
		@apply text-gray-300 hover:cursor-not-allowed hover:text-gray-300;
	}
</style>
