<script lang="ts">
	import Email from '../icons/Email.svelte';
	import TextField from './TextField.svelte';

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
	tooltip="Zadejte platný email.|radko.sablik@ssps.cz"
	{placeholder}
	icon
>
	<div slot="icon" class="flex items-center justify-center">
		<Email />
	</div>
</TextField>

<style>
</style>
