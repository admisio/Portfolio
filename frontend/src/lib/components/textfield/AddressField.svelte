<script lang="ts">
	import { onMount } from 'svelte';
	import TextField from './TextField.svelte';

	export let helperText: string = 'Zadejte jméno a příjmení. Například Radko Sáblík';
	export let placeholder: string = '';

	export let valueLeft: string = '';
	export let valueRight: string = '';

	let value: string = '';

	if (valueLeft && valueRight) {
		value = `${valueLeft} ${valueRight}`;
	} else if (valueLeft) {
		value = valueLeft;
	}

	$: {
		const parsed = value.trim().split(' ');
        console.log(parsed);
		if (parsed.length >= 2) {
			valueLeft = parsed.slice(0, parsed.length - 1).join(' ');
			valueRight = parsed[parsed.length - 1];
		} else {
			valueLeft = parsed[0];
			valueRight = '';
		}
	}

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
