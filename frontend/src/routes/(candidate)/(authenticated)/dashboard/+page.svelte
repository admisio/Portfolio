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
</FullLayout>

<style>
	.dashboardDesktop {
		@apply w-[70vw] h-[70vh];
		@apply hidden md:grid grid-cols-8 grid-rows-2 gap-10;
	}

	.dashboardMobile {
		@apply w-[100vw] h-[90vh];
		@apply md:hidden;
	}

	.dashboardMobile :global(.uploadCard) {
		@apply min-h-72;
	}
</style>
