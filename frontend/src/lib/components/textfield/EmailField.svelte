<script lang="ts">
	import Email from '../icons/Email.svelte';
	import TextField from './TextField.svelte';

	let helperText: string = 'Zadejte platný email. Například radko.sablik@ssps.cz';
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
	on:change
	type="email"
	{placeholder}
	{helperText}
	icon
>
	<div slot="icon" class="flex items-center justify-center">
		<Email />
	</div>
</TextField>

<style>
</style>
