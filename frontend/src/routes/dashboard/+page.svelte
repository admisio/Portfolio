<script lang="ts">
	import FullLayout from '$lib/components/layout/FullLayout.svelte';

	import { Swiper, SwiperSlide } from 'swiper/svelte';
	import 'swiper/css';
</script>

<FullLayout>
	<div class="dashboard dashboardDesktop">
		<div class="card name col-span-2 relative">
			<div class="circles relative -left-16 -top-20">
				<span
					class="shadow-md left-20 top-10 absolute p-1 w-20 h-20 rounded-full bg-black bg-[#f8fbfc] opacity-100"
				/>
				<span
					class="shadow-md left-14 -top-6 absolute p-1 w-24 h-24 rounded-full bg-black bg-[#558cbd] opacity-50"
				/>
				<span
					class="shadow-md absolute p-1 w-28 h-28 rounded-full bg-black bg-[#55b1bd] opacity-60"
				/>
			</div>
			<div class="mt-[50%] flex flex-col">
				<h3>Damián</h3>
				<span class="mt-3 text-sspsBlue truncate">damina.vysin@example.com</span>
				<span class="mt-3 text-sspsGray text-xs">Uchazeč na SSPŠ</span>
			</div>
		</div>
		<div class="card coverletter col-span-5 flex flex-col justify-between">
			<div class="flex flex-row justify-between items-center">
				<h3>Motivační dopis</h3>
				<div class="py-1 px-4 flex bg-white rounded-xl items-center justify-between shadow-md">
					<span class="text-sspsBlue italic text-sm">PDF</span>
					<span class="mx-2 text-sspsGray">/</span>
					<span class="text-sspsBlue italic text-sm">Max 10 MB</span>
				</div>
			</div>
			<div class="drag mt-10 h-full flex flex-col items-center justify-center">
				<span class="text-[#406280] opacity-60 text-sm">Sem přetáhněte,</span>
				<span class="text-sspsGray opacity-60 text-sm">nebo nahrajte svůj motivační dopis</span>
			</div>
		</div>
		<div class="card portfolio col-span-3 flex flex-col justify-between">
			<div class="flex flex-row justify-between items-center">
				<h3>Portfolio</h3>
				<div class="py-1 px-4 flex bg-white rounded-xl items-center justify-between shadow-md">
					<span class="text-sspsBlue italic text-sm">PDF</span>
					<span class="mx-2 text-sspsGray">/</span>
					<span class="text-sspsBlue italic text-sm">Max 10 MB</span>
				</div>
			</div>
			<div class="drag mt-10 h-full flex flex-col items-center justify-center">
				<span class="text-[#406280] opacity-60 text-sm">Sem přetáhněte,</span>
				<span class="text-sspsGray opacity-60 text-sm">nebo nahrajte svůj motivační dopis</span>
			</div>
		</div>
		<div class="card moreData col-span-4 flex flex-col justify-between">
			<div class="flex flex-row justify-between items-center">
				<h3>Další data</h3>
				<div class="py-1 px-4 flex bg-white rounded-xl items-center justify-between shadow-md">
					<span class="text-sspsBlue italic text-sm">ZIP</span>
					<span class="mx-2 text-sspsGray">/</span>
					<span class="text-sspsBlue italic text-sm">Max 100 MB</span>
				</div>
			</div>
			<div class="drag mt-10 h-full flex flex-col items-center justify-center">
				<span class="text-[#406280] opacity-60 text-sm">Sem přetáhněte,</span>
				<span class="text-sspsGray opacity-60 text-sm">nebo nahrajte svůj motivační dopis</span>
			</div>
		</div>
	</div>
	<div class="dashboard dashboardMobile">
		<div class="mt-10 card name relative mb-10">
			<div class="circles relative -left-10 -top-13">
				<span
					class="shadow-md left-20 top-10 absolute p-1 w-20 h-20 rounded-full bg-black bg-[#f8fbfc] opacity-100"
				/>
				<span
					class="shadow-md left-14 -top-6 absolute p-1 w-24 h-24 rounded-full bg-black bg-[#558cbd] opacity-50"
				/>
				<span
					class="shadow-md absolute p-1 w-28 h-28 rounded-full bg-black bg-[#55b1bd] opacity-60"
				/>
			</div>
			<div class="mt-24 flex flex-col">
				<h3 class="!text-3xl">Damián</h3>
				<span class="mt-3 text-sspsBlue truncate">damina.vysin@example.com</span>
				<span class="mt-3 text-sspsGray text-xs">Uchazeč na SSPŠ</span>
			</div>
		</div>
		<Swiper slidesPerView={1} spaceBetween={20} class="h-full">
			{#each [0, 0, 0] as _}
				<SwiperSlide>
					<div class="card min-h-64 coverletter h-full col-span-5 flex flex-col justify-between">
						<div class="flex flex-col sm:flex-row justify-between items-start sm:items-center">
							<h3>Motivační dopis</h3>
							<div
								class="mt-3 sm:mt-0 py-1 px-4 flex bg-white rounded-xl items-center justify-between shadow-md"
							>
								<span class="text-sspsBlue italic text-sm">PDF</span>
								<span class="mx-2 text-sspsGray">/</span>
								<span class="text-sspsBlue italic text-sm">Max 10 MB</span>
							</div>
						</div>
						<div class="flex-grow drag mt-6 h-full flex flex-col items-center justify-center">
							<span class="text-[#406280] opacity-60 text-sm">Sem přetáhněte,</span>
							<span class="text-sspsGray opacity-60 text-sm"
								>nebo nahrajte svůj motivační dopis</span
							>
						</div>
					</div>
				</SwiperSlide>
			{/each}
		</Swiper>
	</div>
</FullLayout>

<style>
	.dashboardDesktop {
		@apply w-[70vw] h-[70vh];
		@apply hidden md:grid grid-cols-7 grid-rows-2 gap-10;
	}
	.dashboardMobile {
		@apply w-[100vw] h-[90vh];
		@apply md:hidden;
	}
	.dashboardMobile .card {
		@apply w-[90%] mx-auto;
	}
	.dashboard .card {
		@apply bg-[#f8fbfc] p-1;
		@apply rounded-3xl;
		@apply px-7 py-10;
	}
	.dashboard .card h3 {
		@apply text-sspsBlue text-2xl xl:text-4xl font-semibold;
	}
	.drag {
		background-image: url("data:image/svg+xml,%3csvg width='100%25' height='100%25' xmlns='http://www.w3.org/2000/svg'%3e%3crect width='100%25' height='100%25' fill='none' rx='9' ry='9'  stroke-opacity='50%' stroke='%23406280' stroke-width='4' stroke-dasharray='10' stroke-dashoffset='0' stroke-linecap='square'/%3e%3c/svg%3e");
		border-radius: 9px;
	}
</style>
