<script lang="ts">
	import TextField from './TextField.svelte';

	export let helperText = 'Zadejte jméno a příjmení. Například Radko Sáblík';
	export let placeholder = '';

	export let valueName = '';
	export let valueSurname = '';

	let value = '';

	if (valueName && valueSurname) {
		value = `${valueName} ${valueSurname}`;
	} else if (valueName) {
		value = valueName;
	}

	$: {
		const parsed = value.trim().split(' ');
		if (parsed.length == 2) {
			valueName = parsed[0];
			valueSurname = parsed[1];
		} else if (parsed.length > 2) {
			valueName = parsed[0];
			// 	valueSurname = parsed[parsed.length - 1];
			// Multiple surnames / names
			valueSurname = parsed.slice(1).join(' ');
		} else {
			valueName = parsed[0];
			valueSurname = '';
		}
	}

	export let error = '';
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
