<script lang="ts">
	export let placeholder: string = '';
	export let value: string = '';

	export let error: string = '';

	export let options: Array<string> = [];
</script>

<div>
	<select
		class:error
		bind:value
		on:click
		on:keydown
		on:keyup
		on:change
		{placeholder}
		class:placeholder={!value}
	>
		{#if placeholder}
			<option value="" disabled selected>{placeholder}</option>
		{/if}
		{#each options as option}
			<option value={option}>{option}</option>
		{/each}
	</select>
</div>

<style lang="postcss">
	div,
	input {
		@apply w-full;
	}
	div {
		@apply relative flex items-center justify-center;
	}

	select {
		@apply hover:border-sspsBlue w-full rounded-lg border border-2 bg-[#f8fafb] p-3 text-xl shadow-lg outline-none transition-colors  duration-300;
		@apply min-w-40;
	}
	option {
		@apply w-full;
		@apply text-center;
	}
	.placeholder {
		@apply text-gray-400;
	}
	.error {
		@apply border-red-700;
	}
	select {
		-webkit-appearance: none;
		-moz-appearance: none;
		appearance: none;
	}
</style>
