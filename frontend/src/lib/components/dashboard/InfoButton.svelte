<script lang="ts">
	import { tippy } from 'svelte-tippy';
	import 'tippy.js/dist/tippy.css';
	import { createEventDispatcher } from 'svelte';
	import { submissionProgress, UploadStatus } from '$lib/stores/portfolio';

	const dispatch = createEventDispatcher();

	const showInfo = () => {
		dispatch('showInfo');
	}

	const download = () => {
		dispatch('download');
	}
</script>

<div class="flex flex-col">
	<div class="flex flex-col">
		<span on:click={(_) => showInfo()} on:keydown={(_) => showInfo()} 
		use:tippy={{
			content: "Zobrazit osobní údaje",
			placement: 'top',
			showOnCreate: false,
			delay: 0
			}}>
			<svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>
		</span>

		{#if $submissionProgress.status === UploadStatus.Submitted}
			<span on:click={(_) => download()} on:keydown={(_) => download()} use:tippy={{
				content: "Stáhnout portfolio",
				placement: 'top',
				showOnCreate: false,
				delay: 0
				}}>
			<svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path></svg>
			</span>
		{:else}
		<span on:click={(_) => download()} on:keydown={(_) => download()} use:tippy={{
			content: "Nelze stáhnout, portfolio nebylo odevzdáno",
			placement: 'top',
			showOnCreate: false,
			delay: 0
			}}>
		<svg class="disabledIcon" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path></svg>
		</span>
		{/if}
	</div>
</div>

<style>
	.icon {
		@apply w-10 h-10 stroke-sspsBlueDark hover:cursor-pointer;
	}
	.disabledIcon {
		@apply w-10 h-10 stroke-gray-300 hover:cursor-not-allowed;
	}
</style>
