<script lang="ts">
	import TextField from './TextField.svelte';

	export let helperText = 'Uveďte ulici a číslo popisné (např. Preslova 72/25).';
	export let placeholder = '';

	export let valueLeft = '';
	export let valueRight = '';

	let value = '';

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
