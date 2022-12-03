<script lang="ts">
	import FullLayout from '$lib/components/layout/FullLayout.svelte';

	import { Swiper, SwiperSlide } from 'swiper/svelte';
	import 'swiper/css';

	import DashboardUploadCard from '$lib/components/dashboard/DashboardUploadCard.svelte';
	import DashboardInfoCard from '$lib/components/dashboard/DashboardInfoCard.svelte';
	import CoverLetterUploadCard from '$lib/components/dashboard/CoverLetterUploadCard.svelte';
	import PortfolioLetterUploadCard from '$lib/components/dashboard/PortfolioLetterUploadCard.svelte';
	import PortfolioZipUploadCard from '$lib/components/dashboard/PortfolioZipUploadCard.svelte';
	import type { PageData } from './$types';
	import { fetchSubmProgress, submissionProgress, UploadStatus, type Status } from '$lib/stores/portfolio';
	import { candidateData } from '$lib/stores/candidate';
	import { apiSubmitPortfolio } from '$lib/@api/candidate';
	import StatusNotificationBig from '$lib/components/dashboard/StatusNotificationBig.svelte';

	export let data: PageData;

	// @ts-ignore
	$: candidateData.set(data.candidate);
	// @ts-ignore
	$: submissionProgress.set(data.submission);


	const submit = async () => {
		const res = await apiSubmitPortfolio();
		await fetchSubmProgress();
	}

	const getUploadStatus = (progressStatus: UploadStatus | undefined): Status => {
		switch (progressStatus) {
			case 3:
				return 'uploaded';
			case 4:
				return 'submitted';
			default:
				return 'missing';
		}
	}
</script>

<FullLayout>
	<div class="flex flex-col">
		<div class="flex flex-row mb-15 m-auto">
			<StatusNotificationBig status={getUploadStatus($submissionProgress.status)}/>
			{#if $submissionProgress.status === 3}
				<button on:click={submit} class="text-2xl h-16 w-96 ml-5 text-blue-gray-700 font-bold bg-white rounded-full">
					Odevzdat soubory
						<svg class="w-8 h-8 inline-block align-middle" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path></svg>
				</button>
			{/if}
		</div>
		<!-- <div class="info-bar col-span-3"></div>
		<div class="col-span-3"></div> -->
		
		<div class="dashboard dashboardDesktop">
			<div class="name col-span-3">
				<DashboardInfoCard title={$candidateData.name + " " + $candidateData.surname ?? ""}>
					<span class="mt-3 text-sspsBlue truncate">{$candidateData.email}</span>
					<span class="mt-3 text-sspsGray text-xs">Uchazeč na SSPŠ</span>
				</DashboardInfoCard>
			</div>
			<div class="coverletter col-span-5">
				<CoverLetterUploadCard />
			</div>
			<div class="portfolio col-span-4">
				<PortfolioLetterUploadCard />
			</div>
			<div class="moreData col-span-4">
				<PortfolioZipUploadCard />
			</div>
		</div>
		<div class="dashboard dashboardMobile">
			<div class="my-10 name w-[90%] mx-auto">
				<DashboardInfoCard title={$candidateData.name + " " + $candidateData.surname ?? ""}>
					<span class="mt-3 text-sspsBlue truncate">{$candidateData.email}</span>
					<span class="mt-3 text-sspsGray text-xs">Uchazeč na SSPŠ</span>
				</DashboardInfoCard>
			</div>
			<Swiper slidesPerView={1} spaceBetween={20}>
				{#each [0, 0, 0] as _}
					<SwiperSlide>
						<div class="w-[90%] mx-auto">
							<DashboardUploadCard title="Motivační dopis" filetype="PDF" filesize="10 MB" />
						</div>
					</SwiperSlide>
				{/each}
			</Swiper>
		</div>
	</div>
</FullLayout>

<style>
	.dashboardDesktop {
		@apply w-[70vw] h-[70vh];
		@apply hidden md:grid grid-cols-8 grid-rows-2 gap-10;
		/* @apply grid-rows-[5%,45%,45%]; */
	}

	.dashboardMobile {
		@apply w-[100vw] h-[90vh];
		@apply md:hidden;
	}

	.dashboardMobile :global(.uploadCard) {
		@apply min-h-72;
	}
</style>
