<script lang="ts">
	import { apiSubmitPortfolio } from '$lib/@api/candidate';
	import Circles from '$lib/components/icons/Circles.svelte';
	import { fetchSubmProgress, type Status } from '$lib/stores/portfolio';
	import StatusNotificationBig from './StatusNotificationBig.svelte';

	export let title: string;
	export let status: Status;

	let uploading = false;

	const submit = async () => {
		uploading = true;
		await apiSubmitPortfolio();
		await fetchSubmProgress();
		uploading = false;
	};

	const handleNotificationClick = async () => {
		if (status === "uploaded") {
			await submit();
		} else if (status === "submitted") {
			
		}
	}
</script>

<div class="card flex flex-col">
	<div class="infoBar flex flex-row-reverse">
		<StatusNotificationBig loading={uploading} {status} on:click={handleNotificationClick} />
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
