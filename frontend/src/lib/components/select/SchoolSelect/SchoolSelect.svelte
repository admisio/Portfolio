<script lang="ts">
	import LL from '$i18n/i18n-svelte';

	import School from './School.svelte';
	import type { School as SchoolType, SchoolJson } from '$lib/stores/candidate';
	import SelectField from '../SelectField.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';

	export let schoolNames: Array<string>;
	export let schoolList: Array<SchoolJson>;

	let fields: Array<string> = [];
	let filteredSchools: Array<string> = [];

	const filterSchools = () => {
		let storageArr: Array<string> = [];
		if (schoolNameInputValue) {
			schoolNames.forEach((school) => {
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
	let fieldFocusInputValue = '';

	$: if (!schoolNameInputValue) {
		filteredSchools = [];
		hiLiteIndex = -1;
	}

	const setFields = (schoolName: string) => {
		let school = schoolList.find((school) => school.n === schoolName);
		if (school) {
			fields = school.f;
		} else {
			fields = [];
		}
	};

	$: setFields(schoolNameInputValue);

	const setInputVal = (schoolName: string) => {
		schoolNameInputValue = removeBold(schoolName);
		filteredSchools = [];
		hiLiteIndex = -1;
		searchInput.focus();
		// setFields(schoolNameInputValue);
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

	let hiLiteIndex = 0;

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
	export let error = '';

	if (selectedSchool.field.split(';').length > 1) {
		console.log(selectedSchool.field);
		schoolFieldInputValue = selectedSchool.field.split(';')[0];
		fieldFocusInputValue = selectedSchool.field.split(';')[1];
	} else {
		schoolFieldInputValue = selectedSchool.field;
	}
	schoolNameInputValue = selectedSchool.name;

	$: selectedSchool.field =
		schoolFieldInputValue + (fieldFocusInputValue ? `;${fieldFocusInputValue}` : '');
	$: selectedSchool.name = schoolNameInputValue;

	let isSSPS = false;
	$: isSSPS = schoolNameInputValue === 'Smíchovská střední průmyslová škola a gymnázium';
</script>

<svelte:window on:keydown={navigateList} />

<div class="autocomplete">
	<div class="flex flex-col">
		<input
			class:error
			type="text"
			bind:this={searchInput}
			bind:value={schoolNameInputValue}
			on:input={filterSchools}
			placeholder={$LL.input.schoolName()}
		/>
		<div class="mt-2 flex">
			<span class="w-1/2" class:w-full={isSSPS}>
				<SelectField
					on:focus={() => setFields(schoolNameInputValue)}
					bind:value={schoolFieldInputValue}
					options={fields}
					placeholder={$LL.input.fieldOfStudy()}
				/>
			</span>
			<span class="ml-2 w-1/2" class:hidden={isSSPS}>
				<TextField
					bind:value={fieldFocusInputValue}
					placeholder="Zaměření (jen některé školy)"
					helperText="Např. Kybernetická bezpečnost, protože obor nemá svůj vlastní kód"
				/>
			</span>
		</div>
		<!-- <select
			on:focus={() => setFields(schoolNameInputValue)}
			>
			{#each fields as field}
				<option>{field}</option>
			{/each}
		</select> -->
		<!-- <input
			on:focus={() => setFields(schoolNameInputValue)}
			class:error
			class="mt-4"
			type="text"
			bind:value={schoolFieldInputValue}
			placeholder={$LL.input.fieldOfStudy()}
		/> -->
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
		@apply w-full rounded-lg border border-2 bg-[#f8fafb] p-3 text-xl shadow-lg outline-none transition-colors duration-300;
		--at-apply: "hover:border-sspsBlue";
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
