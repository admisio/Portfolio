<script lang="ts">
	import LL from '$i18n/i18n-svelte';

	import type { School } from '$lib/stores/candidate';
	// TODO
	// import AutoComplete from 'simple-svelte-autocomplete';
	import { onMount } from 'svelte';
	// import schoollistString from '$lib/assets/schoollist.txt';

	let schools: string[] = [];

	onMount(async () => {
		schools = await fetch('/schoollist.txt')
			.then((response) => response.text())
			.then((text) => text.split(';'));
	});

	export let selectedSchool: School;
	export let schoolName: string = selectedSchool.name;
	$: selectedSchool.name = schoolName;
</script>

<div class="flex flex-row">
	<div>
		<span>
			{$LL.input.selectedSchool()}: {selectedSchool.name}
		</span>
		<!-- TODO -->
		<!-- <AutoComplete items={schools} bind:selectedItem={schoolName} /> -->
		<input type="text" bind:value={schoolName} />
	</div>
	<div class="flex">
		<span>{$LL.input.fieldOfStudy()}: </span>
		<input type="text" bind:value={selectedSchool.field} />
	</div>
</div>
