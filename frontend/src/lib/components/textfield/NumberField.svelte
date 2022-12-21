<script lang="ts">
	import TextField from './TextField.svelte';

	export let helperText: string = '';
	export let placeholder: string = '';
	
	export let value: string = '';
	export let error: string = '';

	// Number formatting
	$: {
		value = value.replace(/[^0-9]/g, '');
	}
</script>

<TextField bind:error bind:value on:keydown on:keyup on:change type="number" {placeholder} {helperText} />
