<script lang="ts">
	export let type: 'text' | 'number' | 'tel' | 'e-mail' | "password" = 'text';
	const typeAction = (node: HTMLInputElement) => {
		node.type = type;
	};
	export let placeholder: string = '';
	export let value: string = '';

	export let icon: boolean = false;
</script>

<div class="mt-8 relative flex justify-center items-center">
	<input
		bind:value
		on:click
		on:keydown
		on:keyup
		class:withIcon={icon}
		class="bg-[#f8fafb] w-full shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
		use:typeAction
		{placeholder}
	/>
	{#if icon}
		<span class="flex absolute right-0 top-0 bottom-0 my-auto bg-transparent p-3">
			<slot name="icon" />
		</span>
	{/if}
</div>

<style>
	.withIcon {
		@apply pr-14;
	}
</style>
