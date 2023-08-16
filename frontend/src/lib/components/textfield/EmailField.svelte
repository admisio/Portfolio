<script lang="ts">
	import Email from '../icons/Email.svelte';
	import TextField from './TextField.svelte';

	let helperText = 'Zadejte platný email. Například radko.sablik@ssps.cz';
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

<style lang="postcss">
</style>
