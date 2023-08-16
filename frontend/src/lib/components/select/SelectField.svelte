<script lang="ts">
	export let placeholder = '';
	export let value = '';

	export let error = '';

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
	select {
		@apply w-full;
	}
	div {
		@apply relative flex items-center justify-center;
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
		@apply  w-full rounded-lg border border-2 bg-[#f8fafb] p-3 text-xl shadow-lg outline-none transition-colors  duration-300;
		@apply min-w-40;
		--at-apply: "hover:border-sspsBlue";

		-webkit-appearance: none !important;
		-moz-appearance: none !important;
		appearance: none !important;
	}
</style>
