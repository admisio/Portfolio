<script lang="ts">
	import Telephone from '../icons/Telephone.svelte';
	import TextField from './TextField.svelte';

	export let placeholder: string = '';
	export let value: string = '';
	export let error: string = '';

	// Personal Id formatting
	$: {
		let x = value.replace(/\D/g, '').match(/(\d{0,6})(\d{0,4})/)!;
		value = x[1] + (x[2] ? '/' + x[2] : '');
	}
</script>

<TextField
	bind:error
	bind:value
	on:keydown
	on:keyup
	on:change
	type="text"
	{placeholder}
	icon
>
	<div slot="icon" class="flex items-center justify-center">
		<Telephone />
	</div>
</TextField>
