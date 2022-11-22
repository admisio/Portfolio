<script lang="ts">
	import FullLayout from '$lib/components/layout/FullLayout.svelte';

	import { Swiper, SwiperSlide } from 'swiper/svelte';
	import 'swiper/css';

	import DashboardUploadCard from '$lib/components/dashboard/DashboardUploadCard.svelte';
	import DashboardInfoCard from '$lib/components/dashboard/DashboardInfoCard.svelte';
</script>

<FullLayout>
	<div class="dashboard dashboardDesktop">
		<div class="name col-span-3">
			<DashboardInfoCard title="Damián">
				<span class="mt-3 text-sspsBlue truncate">damina.vysin@example.com</span>
				<span class="mt-3 text-sspsGray text-xs">Uchazeč na SSPŠ</span>
			</DashboardInfoCard>
		</div>
		<div class="coverletter col-span-5">
			<DashboardUploadCard title="Motivační dopis" filetype="PDF" filesize="10 MB" />
		</div>
		<div class="portfolio col-span-4">
			<DashboardUploadCard title="Portfolio" filetype="PDF" filesize="10 MB" />
		</div>
		<div class="moreData col-span-4">
			<DashboardUploadCard title="Další data" filetype="ZIP" filesize="100 MB" />
		</div>
	</div>
	<div class="dashboard dashboardMobile">
		<div class="my-10 name w-[90%] mx-auto">
			<DashboardInfoCard title="Damián">
				<span class="mt-3 text-sspsBlue truncate">damina.vysin@example.com</span>
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
