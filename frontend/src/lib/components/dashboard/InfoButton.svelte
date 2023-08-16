<script lang="ts">
	import { tippy } from 'svelte-tippy';
	import 'tippy.js/dist/tippy.css';
	import { createEventDispatcher } from 'svelte';
	import { submissionProgress, UploadStatus } from '$lib/stores/portfolio';
	import Document from '../icons/Document.svelte';
	import Download from '../icons/Download.svelte';
	import { sticky } from 'tippy.js';

	export let showDetails: boolean;

	const dispatch = createEventDispatcher();

	const showInfo = () => {
		dispatch('showInfo');
	};

	const download = () => {
		dispatch('download');
	};

	const logout = () => {
		dispatch('logout');
	};
</script>

<button
	on:click={showInfo}
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
</button>

{#if $submissionProgress.status === UploadStatus.Submitted}
	<button
		on:click={download}
		use:tippy={{
			content: 'Stáhnout portfolio',
			placement: 'top',
			showOnCreate: false,
			delay: 0
		}}
		class="icon"
	>
		<Download />
	</button>
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

<button
	use:tippy={{
		content: 'Odhlásit se',
		placement: 'top',
		showOnCreate: false,
		sticky: true,
		plugins: [sticky]
	}}
	on:click={logout}
	class="icon logoutIcon ml-1 hover:cursor-pointer"
>
	<svg
		class="icon logoutIcon h-10 w-10"
		fill="none"
		stroke="currentColor"
		viewBox="0 0 24 24"
		xmlns="http://www.w3.org/2000/svg"
		><path
			stroke-linecap="round"
			stroke-linejoin="round"
			stroke-width="2"
			d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
		/></svg
	>
</button>

<style lang="postcss">
	.icon {
		@apply text-sspsBlueDark h-10 w-10 transition-colors duration-300;
		--at-apply: 'hover:text-sspsBlue hover:cursor-pointer';
	}
	.logoutIcon {
		--at-apply: '2xl:hidden';
	}
	.showDetails {
		@apply text-sspsBlue;
	}
	.disabledIcon {
		@apply text-gray-300;
		--at-apply: 'hover:cursor-not-allowed hover:text-gray-300';
	}
</style>
