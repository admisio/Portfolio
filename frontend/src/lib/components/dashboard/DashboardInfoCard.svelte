<script lang="ts">
	import debounce from 'just-debounce-it';

	import { apiDeltePortfolio, apiGetPortfolio, apiSubmitPortfolio } from '$lib/@api/candidate';
	import Circles from '$lib/components/icons/Circles.svelte';
	import { fetchSubmProgress, type Status } from '$lib/stores/portfolio';
	import StatusNotificationBig from './StatusNotificationBig.svelte';
	import InfoButton from './InfoButton.svelte';
	import { candidateData } from '$lib/stores/candidate';
	import tippy from 'tippy.js';

	export let title: string;
	export let status: Status;

	export let showDetails = true;
	let loading = false;

	const submitPortfolio = async () => {
		loading = true;
		await apiSubmitPortfolio();
		await fetchSubmProgress();
		loading = false;
	};

	const deletePortfolio = async () => {
		loading = true;
		await apiDeltePortfolio();
		await fetchSubmProgress();
		loading = false;
	};

	const handleNotificationClick = async () => {
		if (status === 'uploaded') {
			await submitPortfolio();
		} else if (status === 'submitted') {
			await deletePortfolio();
		}
	};

	const downloadPortfolio = async () => {
		try {
			const portfolioBlob = await apiGetPortfolio();
			const url = window.URL.createObjectURL(new Blob([portfolioBlob]));
			const link = document.createElement('a');
			link.href = url;
			link.setAttribute('download', 'PORTFOLIO' + '_' + $candidateData.candidate.name + '_' + $candidateData.candidate.surname + '.zip');
			document.body.appendChild(link);
			link.click();
		} catch (e) {
			console.log(e);
		}
	}
</script>
<!-- TODO expand on mouse hover?? -->
<!-- <div class="card flex flex-col" on:mouseenter={(_) => showDetails = true} on:mouseleave={(_) => showDetails = false}> -->
<div class="card flex flex-col">
	<div class="infoBar flex flex-row-reverse">
		<StatusNotificationBig {loading} {status} on:click={debounce(handleNotificationClick, 150)} />
		<div class="mr-4">
			<div on:click on:keydown class="flex flex-col">
				<div class="flex flex-col h-20">
					<InfoButton on:download={downloadPortfolio} on:showInfo={(_) => showDetails = !showDetails}></InfoButton>
				</div>
			</div>
		</div>
	</div>
	<div class="relative flex flex-row">
		<div>
			<span class="absolute -left-16 -top-36">
				<Circles />
			</span>
			<div class="mt-8 flex flex-col lg:mt-12">
				<h3>{title}</h3>
				<slot />
			</div>
		</div>
		{#if showDetails}
		<svg class="ml-12 mr-8 h-40 hidden xl:block mt-10" viewBox="0 0 2 80" xmlns="http://www.w3.org/2000/svg">
			<line
				x1="0"
				y="0"
				x2="0"
				y2="80"
				stroke="#406280ff"
				stroke-width="2"
				stroke-dasharray="3"/>
		</svg>
		<div
			use:tippy={{
				content: "<span>Vámi vyplněné osobní údaje</span>",
				allowHTML: true,
				placement: 'top',
				showOnCreate: false,
				delay: 0
			}}
		 class="flex flex-col justify-around mt-10">
			<span>Adresa: <span class="font-bold">{$candidateData.candidate.address}</span></span>
			<span>Datum narození: <span class="font-bold">{$candidateData.candidate.birthdate}</span></span>
			<span>Místo narození: <span class="font-bold">{$candidateData.candidate.birthplace}</span></span>
			<span>Rodné číslo: <span class="font-bold">{$candidateData.candidate.personalIdNumber}</span></span>
			<span>Telefon: <span class="font-bold">{$candidateData.candidate.telephone}</span></span>
		</div>
		<div
			use:tippy={{
				content: "<span>Vámi vyplněné osobní údaje</span>",
				allowHTML: true,
				placement: 'top',
				showOnCreate: false,
				delay: 0
			}}
		 class="ml-10 <xl:ml-4 flex flex-col justify-around mt-10">
			{#each $candidateData.parents as parent}
			<div class="flex flex-col">
				<span class="font-bold text-sspsBlue text-xl">{parent.name + " " + parent.surname}</span>
				<span>Email: <span class="font-bold">{parent.email}</span></span>
				<span>Telefon: <span class="font-bold">{parent.telephone}</span></span>
			</div>
			{/each}
		</div>
		{/if}
	</div>
</div>

<style>
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
