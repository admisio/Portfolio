<script lang="ts">
	import debounce from 'just-debounce-it';

	import { apiDeltePortfolio, apiGetPortfolio, apiSubmitPortfolio } from '$lib/@api/candidate';
	import Circles from '$lib/components/icons/Circles.svelte';
	import { fetchSubmProgress, type Status } from '$lib/stores/portfolio';
	import StatusNotificationBig from './StatusNotificationBig.svelte';
	import InfoButton from './InfoButton.svelte';
	import { baseCandidateData, candidateData } from '$lib/stores/candidate';
	import tippy, {sticky} from 'tippy.js';
	import { goto } from '$app/navigation';

	export let title: string;
	export let status: Status;

	export let showDetails = false;
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
			link.setAttribute(
				'download',
				'PORTFOLIO' +
					'_' +
					$candidateData.candidate.name +
					'_' +
					$candidateData.candidate.surname +
					'.zip'
			);
			document.body.appendChild(link);
			link.click();
		} catch (e) {
			console.log(e);
		}
	};

	const editDetails = async () => {
		goto('/register?edit=true')
	}
</script>

<div class="card flex flex-col">
	<div class="infoBar <2xl:flex-col flex flex-row-reverse">
		<StatusNotificationBig {loading} {status} on:click={debounce(handleNotificationClick, 150)} />
		<div class="mr-4">
			<div on:click on:keydown class="flex flex-col">
				<div class="<2xl:ml-auto <2xl:flex-row <2xl:my-2 flex flex-col">
					<InfoButton
						bind:showDetails
						on:download={downloadPortfolio}
						on:showInfo={(_) => (showDetails = !showDetails)}
					/>
				</div>
			</div>
		</div>
	</div>
	<div class="relative my-2 flex flex-col overflow-hidden">
		<div>
			<span class="absolute -left-16 -top-36">
				<Circles />
			</span>
			<div class="mt-[5%] flex flex-col">
				<h3>{title}</h3>
				<slot />
			</div>
		</div>
		{#if showDetails}
			<div class="overflow-scroll flex justify-between">
				<div>
					<div
						use:tippy={{
							content: '<span>Vámi vyplněné osobní údaje</span>',
							allowHTML: true,
							placement: 'top',
							showOnCreate: false,
							delay: 0
						}}
						class="mt-4 flex flex-col justify-between leading-10"
					>
						<span>Ev. č. přihlášky: <span class="font-bold">{$baseCandidateData.applicationId}</span></span>
						<span>Obor: <span class="font-bold">{$candidateData.candidate.study}</span></span>
						<span>Adresa: <span class="font-bold">{$candidateData.candidate.address}</span></span>
						<span
							>Datum narození: <span class="font-bold">{$candidateData.candidate.birthdate}</span
							></span
						>
						<span
							>Místo narození: <span class="font-bold">{$candidateData.candidate.birthplace}</span
							></span
						>
						<span
							>Rodné číslo: <span class="font-bold">{$candidateData.candidate.personalIdNumber}</span
							></span
						>
						<span>Telefon: <span class="font-bold">{$candidateData.candidate.telephone}</span></span>
					</div>
					<div
						use:tippy={{
							content: '<span>Vámi vyplněné osobní údaje</span>',
							allowHTML: true,
							placement: 'top',
							showOnCreate: false,
							delay: 0
						}}
						class="mt-4 flex flex-col leading-10"
					>
						{#each $candidateData.parents as parent}
							<div class="flex flex-col">
								<span class="text-sspsBlue text-xl font-bold"
									>{parent.name + ' ' + parent.surname}</span
								>
								<span>Email: <span class="font-bold">{parent.email}</span></span>
								<span>Telefon: <span class="font-bold">{parent.telephone}</span></span>
							</div>
						{/each}
					</div>
				</div>
				<span
					use:tippy={{
						content: 'Upravit osobní údaje',
						placement: 'top',
						showOnCreate: false,
						sticky: true,
						plugins: [sticky]
					}}
				 	on:click={(_) => editDetails()} on:keydown={(_) => editDetails()} class="mt-4 hover:cursor-pointer">
					<svg class="w-10 h-10 stroke-sspsBlue" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path></svg>
				</span>
			</div>
		{/if}
	</div>
</div>

<style lang="postcss">
	.card {
		@apply m-3;
		@apply h-full;

		@apply bg-[#f8fbfc];
		@apply rounded-3xl;
		@apply px-7 py-10 <2xl:px-5 <2xl:py-5;

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
