<script lang="ts">
	import FullLayout from '$lib/components/layout/FullLayout.svelte';

	import { Swiper, SwiperSlide } from 'swiper/svelte';
	import 'swiper/css';

	import DashboardInfoCard from '$lib/components/dashboard/DashboardInfoCard.svelte';
	import CoverLetterUploadCard from '$lib/components/dashboard/CoverLetterUploadCard.svelte';
	import PortfolioLetterUploadCard from '$lib/components/dashboard/PortfolioLetterUploadCard.svelte';
	import PortfolioZipUploadCard from '$lib/components/dashboard/PortfolioZipUploadCard.svelte';
	import type { PageData } from './$types';
	import { fetchSubmProgress, submissionProgress, UploadStatus, type Status } from '$lib/stores/portfolio';
	import { candidateData } from '$lib/stores/candidate';

	export let data: PageData;

	// TODO: transition
	let showDetails = true;

	// @ts-ignore
	$: candidateData.set(data.candidate);
	// @ts-ignore
	$: submissionProgress.set(data.submission);
	
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
	<div class="dashboard dashboardDesktop">
		<div class="movable name col-span-3" class:showDetailsInfoCard={showDetails}>
			<DashboardInfoCard bind:showDetails={showDetails} status={getUploadStatus($submissionProgress.status)} title={$candidateData.candidate.name + ' ' + $candidateData.candidate.surname ?? ''}>
				<span class="text-sspsBlue mt-3 truncate">{$candidateData.candidate.email}</span>
				<span class="text-sspsGray mt-3 text-xs">Uchazeč na SSPŠ</span>
			</DashboardInfoCard>
		</div>
		<div class="movable coverletter col-span-5" class:showDetailsUploadCard={showDetails}>
			<CoverLetterUploadCard compact={showDetails} />
		</div>
		<div class="portfolio col-span-4">
			<PortfolioLetterUploadCard />
		</div>
		<div class="moreData col-span-4">
			<PortfolioZipUploadCard />
		</div>
	</div>
	<div class="dashboard dashboardMobile">
		<div class="name my-10 mx-auto w-[90%]">
			<DashboardInfoCard status={getUploadStatus($submissionProgress.status)} title={$candidateData.candidate.name + ' ' + $candidateData.candidate.surname ?? ''}>
				<span class="text-sspsBlue mt-3 truncate">{$candidateData.candidate.email}</span>
				<span class="text-sspsGray mt-3 text-xs">Uchazeč na SSPŠ</span>
			</DashboardInfoCard>
		</div>
		<Swiper slidesPerView={1} spaceBetween={20}>
			<SwiperSlide>
				<div class="mx-auto w-[90%]">
					<CoverLetterUploadCard />
				</div>
			</SwiperSlide>
			<SwiperSlide>
				<div class="mx-auto w-[90%]">
					<PortfolioLetterUploadCard />
				</div>
			</SwiperSlide>
			<SwiperSlide>
				<div class="mx-auto w-[90%]">
					<PortfolioZipUploadCard />
				</div>
			</SwiperSlide>
		</Swiper>
	</div>
</FullLayout>

<style>
	.showDetailsInfoCard {
		@apply col-span-5 <2xl: col-span-6;
	}
	.showDetailsUploadCard {
		@apply col-span-3 <2xl: col-span-2;
	}
	.dashboardDesktop {
		@apply h-[85vh] w-[85vw];
		@apply hidden grid-cols-8 grid-rows-2 gap-10 md:grid;
	}

	.dashboardMobile {
		@apply h-[90vh] w-[100vw];
		@apply md:hidden;
	}

	.dashboardMobile :global(.uploadCard) {
		@apply min-h-72;
	}
</style>
