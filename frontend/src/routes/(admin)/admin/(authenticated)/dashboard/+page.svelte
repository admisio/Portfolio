<script lang="ts">
	import { apiListCandidates } from '$lib/@api/admin';
	import Home from '$lib/components/icons/Home.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';
	import type { CandidatePreview } from '$lib/stores/candidate';
	import CreateCandidateModal from '$lib/components/admin/CreateCandidateModal.svelte';
	import Fuse from 'fuse.js';
	import type { PageServerData } from './$types';

	export let data: PageServerData;

	let candidates: Array<CandidatePreview> = data.preview;

	const getCandidates = async (field?: string) => {
		try {
			candidates = await apiListCandidates(undefined, field);
		} catch {
			console.log('error');
		}
	};

	type Filter = 'Vše' | 'KBB' | 'IT' | 'GYM';

	let filters: Array<Filter> = ['Vše', 'KBB', 'IT', 'GYM'];

	let activeFilter: Filter = 'Vše';

	const changeFilter = (filter: Filter) => {
		activeFilter = filter;
		switch (activeFilter) {
			case 'Vše':
				getCandidates();
				break;
			case 'KBB':
				getCandidates('KB');
				break;
			case 'IT':
				getCandidates('IT');
				break;
			case 'GYM':
				getCandidates('G');
				break;
		}
	};

	let scrollTop = 0;

	let createCandidateModal: boolean = false;

	const openCreateCandidateModal = () => {
		createCandidateModal = true;
	};

	$: candidatesTable = candidates;
	let searchValue: string = '';
	$: fuse = new Fuse(candidates, {
		keys: ['applicationId', 'name', 'surname', 'study']
	});

	const search = () => {
		if (searchValue === '' || !searchValue) {
			candidatesTable = candidates;
		} else {
			candidatesTable = fuse.search(searchValue).map((result) => result.item);
		}
	};
</script>

{#if createCandidateModal}
	<CreateCandidateModal
		on:created={() => getCandidates()}
		on:close={() => (createCandidateModal = false)}
	/>
{/if}

<div>
	<div class="flex flex-row">
		<div class="list fixed">
			{#each filters as filter}
				<div class:selected={filter === activeFilter}>
					<Home />
					<button on:click={() => changeFilter(filter)}>{filter}</button>
				</div>
			{/each}
		</div>
		<div class="body relative overflow-scroll">
			<h1 class="text-3xl font-semibold">Uchazeči</h1>
			<div class="controls my-8">
				<TextField on:keyup={search} bind:value={searchValue} placeholder="Hledat" />
				<button
					on:click={openCreateCandidateModal}
					class="bg-sspsBlue hover:bg-sspsBlueDark ml-3 w-2/5 rounded-lg p-3 py-4 text-xl font-semibold text-white transition-colors duration-300"
					>Nový uchazeč</button
				>
			</div>
			{#if scrollTop > 200}
				<div class="fixed bottom-8 right-8">
					<button
						on:click={openCreateCandidateModal}
						class="bg-sspsBlue flex h-16 w-16 items-center justify-center rounded-full p-6 text-lg font-semibold text-white"
						>+</button
					>
				</div>
			{/if}

			<div class="flex flex-col">
				<div class="overflow-x-auto sm:-mx-6 lg:-mx-8">
					<div class="inline-block min-w-full py-4 sm:px-6 lg:px-8">
						<div class="overflow-hidden rounded-md border-2  border-[#dfe0e9] ">
							<table class="min-w-full text-center ">
								<thead class="bg-[#f6f4f4] ">
									<tr>
										<th scope="col" class="px-6 py-4 text-sm font-medium text-gray-900">
											Ev. č. přihlásky
										</th>
										<th scope="col" class="px-6 py-4 text-sm font-medium text-gray-900"> Jméno </th>
										<th scope="col" class="px-6 py-4 text-sm font-medium text-gray-900">
											Příjmení
										</th>
										<th scope="col" class="px-6 py-4 text-sm font-medium text-gray-900"> Obor </th>
									</tr>
								</thead>
								<tbody>
									{#each candidatesTable as candidate}
										<tr class="border-b bg-white hover:cursor-pointer">
											<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-900"
												><a
													target="_blank"
													rel="noreferrer"
													href="/admin/candidate/{candidate.applicationId}"
													>{candidate.applicationId}</a
												></td
											>
											<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-900">
												{candidate.name}
											</td>
											<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-900">
												{candidate.surname}
											</td>
											<td class="whitespace-nowrap px-6 py-4 text-sm text-gray-900">
												{candidate.study}
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

<svelte:window on:scroll={() => (scrollTop = window.scrollY)} />

<style lang="postcss">
	.list {
		@apply h-full w-96;
		@apply float-left overflow-scroll;

		@apply border-r border-gray-400;
		@apply bg-white;
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
		@apply h-full w-full;
		@apply float-left overflow-hidden;
		@apply my-6 mx-12 ml-[27rem];
	}

	.body .controls {
		@apply flex flex-row items-center justify-between;
	}
</style>
