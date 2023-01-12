<script lang="ts">
	import { apiDeleteCandidate, apiListCandidates, apiListCandidatesCSV } from '$lib/@api/admin';
	import Home from '$lib/components/icons/Home.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';
	import type { CandidatePreview } from '$lib/stores/candidate';
	import CreateCandidateModal from '$lib/components/admin/modal/CreateCandidateModal.svelte';
	import Fuse from 'fuse.js';
	import type { PageServerData } from './$types';
	import Table from '$lib/components/admin/table/Table.svelte';

	import bacgkround from '$lib/assets/background.jpg';
	import Logout from '$lib/components/icons/Logout.svelte';
	import { goto } from '$app/navigation';

	export let data: PageServerData;

	let candidates: Array<CandidatePreview> = data.preview;

	const getCandidates = async (field?: string) => {
		try {
			candidates = await apiListCandidates(
				undefined,
				field ?? activeFilter !== 'Vše' ? activeFilter : ''
			);
		} catch {
			console.log('error');
		}
	};

	type Filter = 'Vše' | 'KBB' | 'IT' | 'GYM';

	let filters: Array<Filter> = ['Vše', 'KBB', 'IT', 'GYM'];

	let activeFilter: Filter = filters[0];

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

	const deleteCandidate = async (id: number | undefined) => {
		if (id) await apiDeleteCandidate(id);
		getCandidates();
	};

	const downloadCSV = async () => {
		try {
			const csvBlob = await apiListCandidatesCSV();
			const url = window.URL.createObjectURL(new Blob([csvBlob]));
			const link = document.createElement('a');
			link.href = url;
			link.setAttribute('download', 'UCHAZECI' + '.csv');
			link.click();
		} catch (e) {
			console.log(e);
		}
	};

	const logout = async () => {
		goto('/admin/auth/logout');
	};
</script>

{#if createCandidateModal}
	<CreateCandidateModal
		on:created={() => getCandidates()}
		on:close={() => (createCandidateModal = false)}
	/>
{/if}

<div>
	<header class="absolute h-14 w-full">
		<img class="h-12 w-full object-cover blur-sm filter" src={bacgkround} alt="Background" />
	</header>
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
			<div class="flex items-center">
				<h1 class="text-3xl font-semibold">Uchazeči</h1>
				<button class="ml-2" on:click={logout}>
					<Logout />
				</button>
			</div>
			<div class="controls my-8">
				<TextField on:keyup={search} bind:value={searchValue} placeholder="Hledat" />
				<button
					on:click={openCreateCandidateModal}
					class="ml-3 w-2/5 rounded-lg bg-gray-500 p-3 py-4 text-xl font-semibold text-white transition-colors duration-300 hover:bg-gray-600"
					>Nový uchazeč</button
				>
				<button
					on:click={downloadCSV}
					class="ml-3 w-2/5 rounded-lg bg-gray-500 p-3 py-4 text-xl font-semibold text-white transition-colors duration-300 hover:bg-gray-600"
					>CSV</button
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

			<Table candidates={candidatesTable} on:delete={(event) => deleteCandidate(event.detail.id)} />
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
		@apply my-6 mx-12 mt-16 ml-[27rem];
	}

	.body .controls {
		@apply flex flex-row items-center justify-between;
	}
</style>
