<script lang="ts">
	import TextField from './TextField.svelte';

	export let helperText = '';
	export let placeholder = '';

	export let value = '';
	export let error = '';

	// Number formatting
	$: {
		value = value.replace(/[^0-9]/g, '');
	}
</script>

<TextField
	bind:error
	bind:value
	on:keydown
	on:keyup
	on:change
	type="number"
	{placeholder}
	{helperText}
/>
