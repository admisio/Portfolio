<script lang="ts">
	import { flip } from 'svelte/animate';
	import FullLayout from '$lib/components/layout/FullLayout.svelte';

	import { Swiper, SwiperSlide } from 'swiper/svelte';
	import 'swiper/css';

	import DashboardInfoCard from '$lib/components/dashboard/DashboardInfoCard.svelte';
	import CoverLetterUploadCard from '$lib/components/dashboard/CoverLetterUploadCard.svelte';
	import PortfolioLetterUploadCard from '$lib/components/dashboard/PortfolioLetterUploadCard.svelte';
	import PortfolioZipUploadCard from '$lib/components/dashboard/PortfolioZipUploadCard.svelte';
	import type { PageData } from './$types';
	import {
		fetchSubmProgress,
		submissionProgress,
		UploadStatus,
		type Status
	} from '$lib/stores/portfolio';
	import { baseCandidateData, candidateData } from '$lib/stores/candidate';

	export let data: PageData;

	// TODO: transition
	let showDetails = false;

	$: candidateData.set(data.candidate);
	$: submissionProgress.set(data.submission);
	$: baseCandidateData.set(data.whoami);

	const getUploadStatus = (progressStatus: UploadStatus | undefined): Status => {
		switch (progressStatus) {
			case 3:
				return 'uploaded';
			case 4:
				return 'submitted';
			default:
				return 'missing';
		}
	};
</script>

<FullLayout>
	<div class="dashboard dashboardDesktop">
		{#each [0] as animated (animated)}
			<div
				class="movable name col-span-3 row-span-4"
				animate:flip={{ duration: 400 }}
				class:showDetailsInfoCard={showDetails}
			>
				<DashboardInfoCard
					bind:showDetails
					status={getUploadStatus($submissionProgress.status)}
					title={$candidateData.candidate.name + ' ' + $candidateData.candidate.surname ?? ''}
				>
					<span class="text-sspsBlue mt-3 truncate">{$candidateData.candidate.email}</span>
					<span class="text-sspsGray mt-3 text-xs">Uchazeč na SSPŠ</span>
				</DashboardInfoCard>
			</div>
		{/each}

		<div class="movable coverletter col-span-5 row-span-4">
			<CoverLetterUploadCard />
		</div>
		{#each [0] as animated (animated)}
			<div
				animate:flip={{ duration: 400 }}
				class="portfolio col-span-4 row-span-4"
				class:showDetailsPortfolio={showDetails}
			>
				<PortfolioLetterUploadCard />
			</div>
		{/each}

		{#each [0] as animated (animated)}
			<div
				animate:flip={{ duration: 400 }}
				class="moreData col-span-4 row-span-4"
				class:showDetailsMoreData={showDetails}
			>
				<PortfolioZipUploadCard />
			</div>
		{/each}
	</div>
	<div class="dashboard dashboardMobile">
		<div class="name mx-auto w-[90%] md:my-10">
			<DashboardInfoCard
				status={getUploadStatus($submissionProgress.status)}
				title={$candidateData.candidate.name + ' ' + $candidateData.candidate.surname ?? ''}
			>
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

<style lang="postcss">
	.showDetailsInfoCard {
		@apply md:row-span-8;
	}
	.showDetailsPortfolio {
		@apply md:hidden;
	}
	.showDetailsMoreData {
		@apply md:col-span-5;
	}
	.dashboardDesktop {
		@apply h-[85vh] w-[85vw];
		@apply grid-rows-8 hidden grid-cols-8 gap-10 md:grid;
	}

	.dashboardMobile {
		@apply h-[90vh] w-[100vw];
		@apply md:hidden;
	}

	.dashboardMobile :global(.uploadCard) {
		@apply min-h-72;
	}
</style>
