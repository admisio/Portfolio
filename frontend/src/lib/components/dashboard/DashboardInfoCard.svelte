<script lang="ts">
	import debounce from 'just-debounce-it';

	import { apiDeltePortfolio, apiGetPortfolio, apiSubmitPortfolio } from '$lib/@api/candidate';
	import Circles from '$lib/components/icons/Circles.svelte';
	import { fetchSubmProgress, type Status } from '$lib/stores/portfolio';
	import StatusNotificationBig from './StatusNotificationBig.svelte';
	import InfoButton from './InfoButton.svelte';
	import { candidateData } from '$lib/stores/candidate';

	export let title: string;
	export let status: Status;

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

<div class="card flex flex-col">
	<div class="infoBar flex flex-row-reverse">
		<StatusNotificationBig {loading} {status} on:click={debounce(handleNotificationClick, 150)} />
		<div class="mr-4">
			<div on:click on:keydown class="flex flex-col">
				<div class="flex flex-col h-20">
					<InfoButton on:download={downloadPortfolio}></InfoButton>
				</div>
			</div>
		</div>
	</div>
	<div class="relative flex flex-row justify-between">
		<div>
			<span class="absolute -left-16 -top-36">
				<Circles />
			</span>
			<div class="mt-8 flex flex-col lg:mt-12">
				<h3>{title}</h3>
				<slot />
			</div>
		</div>
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
