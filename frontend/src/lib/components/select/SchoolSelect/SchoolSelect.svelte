<script lang="ts">
	import LL from '$i18n/i18n-svelte';

	import schoollistString from '$lib/assets/schoollist.txt?raw';
	import School from './School.svelte';
	import type { School as SchoolType } from '$lib/stores/candidate';

	const schoolList: Array<string> = schoollistString.split(';');

	let filteredSchools: Array<string> = [];

	const filterSchools = () => {
		let storageArr: Array<string> = [];
		if (schoolNameInputValue) {
			schoolList.forEach((school) => {
				if (
					school
						.toLowerCase()
						.normalize('NFD')
						.replace(/[\u0300-\u036f]/g, '')
						.includes(
							schoolNameInputValue
								.toLowerCase()
								.normalize('NFD')
								.replace(/[\u0300-\u036f]/g, '')
						)
				) {
					storageArr = [...storageArr, makeMatchBold(school)];
				}
			});
		}
		filteredSchools = storageArr;
	};

	let searchInput: HTMLInputElement;
	let optionsList: HTMLUListElement;

	let schoolNameInputValue = '';
	let schoolFieldInputValue = '';

	$: if (!schoolNameInputValue) {
		filteredSchools = [];
		hiLiteIndex = -1;
	}

	const setInputVal = (schoolName: string) => {
		schoolNameInputValue = removeBold(schoolName);
		filteredSchools = [];
		hiLiteIndex = -1;
		searchInput.focus();
	};

	const makeMatchBold = (str: string) => {
		let matched = str.substring(0, schoolNameInputValue.length);
		let makeBold = `<strong>${matched}</strong>`;
		let boldedMatch = str.replace(matched, makeBold);
		return boldedMatch;
	};

	const removeBold = (str: string) => {
		return str.replace(/<(.)*?>/g, '');
	};

	let hiLiteIndex: number = 0;

	const navigateList = (e: KeyboardEvent) => {
		if (e.key === 'ArrowDown') {
			if (hiLiteIndex < filteredSchools.length - 1) {
				hiLiteIndex++;
				// scroll optionsList
				let option = optionsList.children[hiLiteIndex];
				if (option) {
					option.scrollIntoView({ block: 'nearest' });
				}
			}
		} else if (e.key === 'ArrowUp') {
			if (hiLiteIndex > 0) {
				hiLiteIndex--;
			}
		} else if (e.key === 'Enter') {
			if (hiLiteIndex > -1) {
				setInputVal(filteredSchools[hiLiteIndex]);
			}
		}
	};

	export let selectedSchool: SchoolType;
	export let error: string = '';

	schoolFieldInputValue = selectedSchool.field;
	schoolNameInputValue = selectedSchool.name;

	$: selectedSchool.field = schoolFieldInputValue;
	$: selectedSchool.name = schoolNameInputValue;
</script>

<svelte:window on:keydown={navigateList} />

<div class="autocomplete">
	<div class="flex">
		<input
			class:error
			class="flex-1"
			type="text"
			bind:this={searchInput}
			bind:value={schoolNameInputValue}
			on:input={filterSchools}
			placeholder={$LL.input.schoolName()}
		/>
		<input
			class:error
			class="ml-2 w-2/5"
			type="text"
			bind:value={schoolFieldInputValue}
			placeholder={$LL.input.fieldOfStudy()}
		/>
	</div>
	{#if filteredSchools.length > 0}
		<ul bind:this={optionsList} class="schoolAutocompleteList">
			{#each filteredSchools as country, i}
				<School
					itemLabel={country}
					highlighted={i === hiLiteIndex}
					on:click={() => setInputVal(country)}
				/>
			{/each}
		</ul>
	{/if}
</div>

<style lang="postcss">
	div,
	input {
		@apply w-full;
	}
	div {
		@apply relative flex items-center justify-center;
	}
	input {
		@apply hover:border-sspsBlue w-full rounded-lg border border-2 bg-[#f8fafb] p-3 text-xl shadow-lg outline-none transition-colors  duration-300;
	}

	.error {
		@apply border-red-700;
	}

	.autocomplete {
		@apply relative;
	}

	.schoolAutocompleteList {
		@apply absolute top-20 z-50;
		@apply w-full;
		@apply max-h-72 overflow-scroll;
	}
</style>
