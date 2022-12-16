<script lang="ts">
	import Telephone from '../icons/Telephone.svelte';
	import TextField from './TextField.svelte';

	export let placeholder: string = '';
	export let value: string = '';
	export let error: string = '';

	// Phone Number formatting
	$: {
		let x = value.replace(/\D/g, '').match(/(\d{0,3})(\d{0,3})(\d{0,3})(\d{0,3})/)!;
		value =
			(x[1] ? '+' + x[1] : '') +
			(x[2] ? ' ' + x[2] : '') +
			(x[3] ? ' ' + x[3] : '') +
			(x[4] ? ' ' + x[4] : '');
	}
</script>

<TextField bind:error bind:value on:keydown on:keyup on:change type="tel"
tooltip="Zadejte platné telefonní číslo.|+420 123 456 789"
 {placeholder} icon>
	<div slot="icon" class="flex items-center justify-center">
		<Telephone />
	</div>
</TextField>

<style>
</style>
