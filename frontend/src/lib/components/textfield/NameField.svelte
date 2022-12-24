<script lang="ts">
	import { onMount } from 'svelte';
	import TextField from './TextField.svelte';

	let helperText: string = 'Zadejte jméno a příjmení. Například Radko Sáblík';
	export let placeholder: string = '';

	export let valueName: string = '';
	export let valueSurname: string = '';

	let value: string = '';

	$: {
		const parsed = value.trim().split(' ');
		if (parsed.length > 1) {
			valueName = parsed[0];
			valueSurname = parsed[1];
		}
	}

	onMount(() => {
		if (valueName && valueSurname) {
			value = `${valueName} ${valueSurname}`;
		}
	});

	export let error: string = '';
</script>

<TextField
	bind:error
	bind:value
	on:click
	on:keydown
	on:keyup
	on:change
	type="text"
	{placeholder}
	{helperText}
/>

<style lang="postcss">
</style>
