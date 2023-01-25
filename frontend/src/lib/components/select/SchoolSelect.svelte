<script lang="ts">
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
            Selected school: {selectedSchool.name}
        </span>
        <!-- TODO -->
        <!-- <AutoComplete items={schools} bind:selectedItem={schoolName} /> -->
    </div>
    <div class="flex">
        <span>Obor: </span>
        <input type="text" bind:value={selectedSchool.field} />
    </div>
</div>