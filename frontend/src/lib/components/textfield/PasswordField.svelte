<script lang="ts">
	import Lock from '../icons/Lock.svelte';
	import TextField from './TextField.svelte';

	export let helperText: string = '';
	export let placeholder: string = '';

	export let value: string = '';
	export let error: string = '';
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
