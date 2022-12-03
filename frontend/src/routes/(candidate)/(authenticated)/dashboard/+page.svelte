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
	import { fetchSubmProgress, submissionProgress } from '$lib/stores/portfolio';
	import { candidateData } from '$lib/stores/candidate';

	export let data: PageData;

	// @ts-ignore
	$: candidateData.set(data.candidate);
	// @ts-ignore
	$: submissionProgress.set(data.submission);
</script>

<FullLayout>
	<div class="dashboard dashboardDesktop">
		<div class="name col-span-3">
			<DashboardInfoCard title={$candidateData.name + ' ' + $candidateData.surname ?? ''}>
				<span class="text-sspsBlue mt-3 truncate">{$candidateData.email}</span>
				<span class="text-sspsGray mt-3 text-xs">Uchazeč na SSPŠ</span>
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
		<div class="name my-10 mx-auto w-[90%]">
			<DashboardInfoCard title={$candidateData.name + ' ' + $candidateData.surname ?? ''}>
				<span class="text-sspsBlue mt-3 truncate">{$candidateData.email}</span>
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
