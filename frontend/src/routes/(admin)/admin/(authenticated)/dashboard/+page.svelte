<script lang="ts">
	import backgroundImage from '$lib/assets/background.jpg';

	import { apiFetchCandidate, apiListCandidates } from '$lib/@api/admin';
	import Home from '$lib/components/icons/Home.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';
	import type { CandidatePreview } from '$lib/stores/candidate';

	let candidates: [CandidatePreview] = [{}];
	let candidateDetails: { [id: number]: CandidatePreview } = {};
	let currentCandidateId: number = 0;

	getCandidates();

	async function getCandidates() {
		try {
			candidates = await apiListCandidates();
		} catch {
			console.log('error');
		}
	}

	async function getCandidateDetails(id: number) {
		currentCandidateId = id;
		candidateDetails[id] = await apiFetchCandidate(id);
	}

	type Filter = 'Vše' | 'KBB' | 'IT' | 'GYM';

	let filters: Array<Filter> = ['Vše', 'KBB', 'IT', 'GYM'];

	let activeFilter: Filter = 'Vše';
</script>

<div>
	<div class="header" style={`background-image: url(${backgroundImage});`} />
	<div class="flex flex-row">
		<div class="list">
			{#each filters as filter}
				<div class:selected={filter === activeFilter}>
					<Home />
					<button on:click={() => (activeFilter = filter)}>{filter}</button>
				</div>
			{/each}
		</div>
		<div class="body overflow-scroll">
			<h1 class="text-3xl font-semibold">Uchazeči</h1>
			<div class="controls my-8">
				<TextField placeholder="Hledat" />
				<button
					class="bg-sspsBlue hover:bg-sspsBlueDark ml-3 w-2/5 rounded-lg p-3 py-4 text-xl font-semibold text-white transition-colors duration-300"
					>Nový uchazeč</button
				>
			</div>
			<div class="flex flex-col">
				<div class="overflow-x-auto sm:-mx-6 lg:-mx-8">
					<div class="inline-block min-w-full py-4 sm:px-6 lg:px-8">
						<div class="overflow-hidden">
							<table class="min-w-full rounded-md border-2  border-[#dfe0e9] text-center">
								<thead class="bg-[#f6f4f4] ">
									<tr>
										<th scope="col" class="px-6 py-4 text-sm font-medium text-gray-900"> # </th>
										<th scope="col" class="px-6 py-4 text-sm font-medium text-gray-900"> First </th>
										<th scope="col" class="px-6 py-4 text-sm font-medium text-gray-900"> Last </th>
										<th scope="col" class="px-6 py-4 text-sm font-medium text-gray-900">
											Handle
										</th>
									</tr>
								</thead>
								<tbody>
									{#each Array(40) as item}
										<tr class="border-b bg-white">
											<td class="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900"
												>1</td
											>
											<td class="whitespace-nowrap px-6 py-4 text-sm font-light text-gray-900">
												Mark
											</td>
											<td class="whitespace-nowrap px-6 py-4 text-sm font-light text-gray-900">
												Otto
											</td>
											<td class="whitespace-nowrap px-6 py-4 text-sm font-light text-gray-900">
												@mdo
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.header {
		@apply h-16 w-full;
	}
	.list {
		@apply h-[100vh] w-96;
		@apply float-left overflow-scroll;

		@apply border-r border-gray-400;
	}

	.list div {
		@apply p-3;
		@apply mx-3 my-6;
		@apply flex items-center;
		@apply rounded-xl;

		@apply transition-all duration-300;

		@apply hover:bg-sspsBlue focus:bg-sspsBlue;
		@apply hover:text-white focus:text-white;
	}

	.list div :global(path) {
		@apply transition-all duration-300;
	}

	.list div:hover :global(path) {
		@apply fill-white fill-white;
	}
	.list div:hover :global(path:nth-child(2)) {
		@apply stroke-white stroke-white;
	}

	.list .selected :global(path) {
		@apply fill-white fill-white;
	}
	.list .selected :global(path:nth-child(2)) {
		@apply stroke-white stroke-white;
	}

	.list .selected {
		@apply bg-sspsBlue;
		@apply text-white;
	}
	.list div button {
		@apply p-1;
		@apply flex-1;
		@apply text-left;
	}

	.body {
		@apply h-[100vh] w-[calc(100vw-96px)];
		@apply float-left overflow-hidden;
		@apply my-6 mx-10;
	}

	.body .controls {
		@apply flex flex-row items-center justify-between;
	}

	.candidatePreview {
		@apply mt-5 h-20 w-full rounded-xl bg-gray-200;
		@apply hover:cursor-pointer;
	}
</style>
