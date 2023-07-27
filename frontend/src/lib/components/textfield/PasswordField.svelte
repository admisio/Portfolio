<script lang="ts">
	import Lock from '../icons/Lock.svelte';
	import TextField from './TextField.svelte';

	export let helperText = '';
	export let placeholder = '';

	export let value = '';
	export let error = '';
</script>

<TextField
	bind:error
	bind:value
	on:click
	on:keydown
	on:keyup
	{placeholder}
	{helperText}
	on:change
	type="password"
	icon
>
	<div slot="icon" class="flex items-center justify-center">
		<Lock />
	</div>
</TextField>

<style lang="postcss">
	div {
		@apply text-sspsBlue;
	}
</style>
