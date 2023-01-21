<script lang="ts">
import AutoComplete from 'simple-svelte-autocomplete';
import { onMount } from 'svelte';
// import schoollistString from '$lib/assets/schoollist.txt';

let schools: string[] = [];

onMount(async () => {
   schools = await fetch('/schoollist.txt')
    .then((response) => response.text())
    .then((text) => text.split(';'));
});

export let selectedSchool: string = '';
</script>

Selected color: {selectedSchool}
<AutoComplete items={schools} bind:selectedItem={selectedSchool} />