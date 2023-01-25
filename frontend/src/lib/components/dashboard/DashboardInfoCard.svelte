<script lang="ts">
	import LL from '$i18n/i18n-svelte';

	import debounce from 'just-debounce-it';

	import {
		apiDeltePortfolio,
		apiGetPortfolio,
		apiLogout,
		apiSubmitPortfolio
	} from '$lib/@api/candidate';
	import { fetchSubmProgress, type Status } from '$lib/stores/portfolio';
	import StatusNotificationBig from './StatusNotificationBig.svelte';
	import InfoButton from './InfoButton.svelte';
	import { baseCandidateData, candidateData } from '$lib/stores/candidate';
	import tippy, { sticky } from 'tippy.js';
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

	const getField = (id: number) => {
		if (id.toString().startsWith('101')) {
			return 'G';
		} else if (id.toString().startsWith('102')) {
			return 'IT';
		} else {
			return 'KB';
		}
	};

	const editDetails = async () => {
		goto('/register?edit=true');
	};

	const logout = async () => {
		await apiLogout();
		goto('/auth/login');
	};
</script>

<div class="card flex flex-col">
	<div class="infoBar <2xl:flex-col flex flex-row-reverse">
		<StatusNotificationBig {loading} {status} on:click={debounce(handleNotificationClick, 150)} />
		<div class="<2xl:mr-1 mr-4">
			<div on:click on:keydown class="flex flex-col">
				<div class="<2xl:ml-auto <2xl:flex-row <2xl:my-2 flex flex-col">
					<InfoButton
						bind:showDetails
						on:download={downloadPortfolio}
						on:showInfo={(_) => (showDetails = !showDetails)}
						on:logout={logout}
					/>
				</div>
			</div>
		</div>
	</div>
	<div class="relative my-2 flex flex-col overflow-hidden">
		<div>
			<div class="mt-[5%] flex flex-col">
				<div class="flex justify-between">
					<h3>{title}</h3>
					<span
						on:click={logout}
						on:keydown={logout}
						use:tippy={{
							content: 'Odhlásit se',
							placement: 'top',
							showOnCreate: false,
							sticky: true,
							plugins: [sticky]
						}}
						class="<2xl:hidden hover:cursor-pointer"
					>
						<svg
							class="stroke-sspsBlueDark h-10 w-10"
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
					</span>
				</div>
				<slot />
			</div>
		</div>
		{#if showDetails}
			<div class="flex justify-between overflow-scroll">
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
						<span
							>{$LL.input.evidenceNumber()} ({getField($baseCandidateData.applications[0])}):
							<span class="font-bold">{$baseCandidateData.applications[0]}</span></span
						>
						{#if $baseCandidateData.applications.length > 1}
							<span
								>{$LL.input.evidenceNumber()} ({getField($baseCandidateData.applications[1])}):
								<span class="font-bold">{$baseCandidateData.applications[1]}</span></span
							>
						{/if}
						<span>{$LL.input.address()}: <span class="font-bold">{$candidateData.candidate.address}</span></span>
						<span
							>{$LL.input.birthDate()}: <span class="font-bold">{$candidateData.candidate.birthdate}</span
							></span
						>
						<span
							>{$LL.input.birthPlace()}: <span class="font-bold">{$candidateData.candidate.birthplace}</span
							></span
						>
						<span
							>{$LL.input.personalIdentificationNumber()}: <span class="font-bold"
								>{$candidateData.candidate.personalIdNumber}</span
							></span
						>
						<span
							>{$LL.input.schoolIzo()}: <span class="font-bold">{$candidateData.candidate.schoolName}</span
							></span
						>
						<span
							>{$LL.input.insuranceNumber()}: <span class="font-bold"
								>{$candidateData.candidate.healthInsurance}</span
							></span
						>
						<span>{$LL.input.telephone()}: <span class="font-bold">{$candidateData.candidate.telephone}</span></span
						>
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
								<span>{$LL.input.email()}: <span class="font-bold">{parent.email}</span></span>
								<span>{$LL.input.telephone()}: <span class="font-bold">{parent.telephone}</span></span>
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
					on:click={(_) => editDetails()}
					on:keydown={(_) => editDetails()}
					class="mt-4 hover:cursor-pointer"
				>
					<svg
						class="stroke-sspsBlue h-10 w-10"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						xmlns="http://www.w3.org/2000/svg"
						><path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
						/></svg
					>
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
		@apply <2xl:px-5 <2xl:py-5 px-7 py-10;

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
