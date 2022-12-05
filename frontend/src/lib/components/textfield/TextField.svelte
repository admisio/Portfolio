<script lang="ts">
	export let type: 'text' | 'number' | 'tel' | 'email' | 'password' = 'text';
	const typeAction = (node: HTMLInputElement) => {
		node.type = type;
	};
	export let placeholder: string = '';
	export let value: string = '';

	export let icon: boolean = false;
	export let error: string = '';
</script>

<div>
	<input
		class:error
		bind:value
		on:click
		on:keydown
		on:keyup
		on:change
		class:withIcon={icon}
		use:typeAction
		{placeholder}
	/>
	{#if icon}
		<span>
			<slot name="icon" />
		</span>
	{/if}
</div>

<style>
	div,
	input {
		@apply w-full;
	}
	div {
		@apply relative flex items-center justify-center;
	}
	input {
		@apply hover:border-sspsBlue w-full rounded-lg border border-2 bg-[#f8fafb] p-3 text-xl shadow-lg outline-none transition-colors  duration-300;
	}
	div span {
		@apply absolute right-0 top-0 bottom-0 my-auto flex bg-transparent p-3;
	}
	.withIcon {
		@apply pr-14;
	}
	.error {
		@apply border-red-700;
	}
</style>
