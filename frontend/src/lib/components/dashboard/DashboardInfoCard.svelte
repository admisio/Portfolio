<script lang="ts">
	import { apiSubmitPortfolio } from '$lib/@api/candidate';
	import Circles from '$lib/components/icons/Circles.svelte';
	import { fetchSubmProgress, type Status } from '$lib/stores/portfolio';
	import StatusNotificationBig from './StatusNotificationBig.svelte';

	export let title: string;
	export let status: Status;

	const submit = async () => {
		const res = await apiSubmitPortfolio();
		await fetchSubmProgress();
	}
</script>

<div class="card flex flex-col">
	<div class="infoBar flex flex-row-reverse">
		<StatusNotificationBig {status} />
	</div>
	<div class="flex flex-row justify-between relative">
		<div>
			<span class="absolute -left-16 -top-36">
				<Circles />
			</span>
			<div class="mt-20 flex flex-col">
				<h3>{title}</h3>
				<slot />
			</div>
		</div>
		<div class="flex flex-col justify-evenly w-[50%]">
			{#if status === 'uploaded'}
				<button on:click={submit} class="bg-green-600">
					Odevzdat
					<svg class="w-8 h-8 inline-block align-middle" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path></svg>
				</button>
			{:else if status === 'submitted'}
				<button class="bg-gray-500 hover:bg-gray-800">
					Smazat portfolio
					<svg class="w-8 h-8 inline-block align-middle" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
				</button>
			{:else}
				<button class="bg-gray-500 hover:bg-gray-500 hover:cursor-not-allowed">Odevzdat</button>
			{/if}
			<button>
				Zobrazit údaje
				<svg class="w-8 h-8 inline-block align-middle" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>
			</button>
		</div>
	</div>
</div>

<style>
	button {
		@apply bg-sspsBlue text-white font-bold;
		@apply hover:bg-sspsBlueDark transition-colors duration-300;
		@apply text-xl;
		@apply rounded-xl shadow-lg;
		@apply py-3 px-2;
	}
	.card {
		@apply m-3;
		@apply h-full;

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
		@apply text-sspsBlue text-4xl font-semibold md:text-2xl xl:text-4xl;
	}
</style>
