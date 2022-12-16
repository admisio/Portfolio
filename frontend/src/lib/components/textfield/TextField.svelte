<script lang="ts">
	export let type: 'text' | 'number' | 'tel' | 'email' | 'password' = 'text';
	const typeAction = (node: HTMLInputElement) => {
		node.type = type;
	};
	export let placeholder: string = '';
	export let value: string = '';

	export let icon: boolean = false;
	export let error: string = '';
	export let tooltip: string = '';
</script>

<div class="flex flex-row justify-around group">
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
	{#if error}
		<div role="tooltip" class="tooltip group-hover:opacity-100">
			<div class="py-1 flex flex-col">
				<span class="w-full h-5 text-sm font-bold absolute top-0">{tooltip.split("|")[0]}</span>
				<span class="w-full h-5 text-sm absolute top-4">např. {tooltip.split("|")[1]}</span>
			</div>
		</div>
		<!-- <span class="bg-red-700 w-full rounded-xl text-lg text-white">{tooltip}</span>
		<div class="flex flex-col inline-flex bg-red-700 h-[58px] w-[70%] ml-4 rounded-lg">
			<div class="h-[50%] mb-3">
				<span class="text-white text-md">{tooltip.split("|")[0]}</span>
			</div>
			<div class="h-[50%] mb-6">
				<span class="text-white text-sm font-bold">např. {tooltip.split("|")[1]}</span>
			</div>
		</div> -->
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

	.tooltip {
		@apply inline-block text-white opacity-0 items-center
		@apply bg-red-700 w-75 h-14 z-20
		@apply absolute -top-12 rounded-xl;
	}
	.tooltip::after {
		content: "";
		position: absolute;
		top: 100%;
		left: 50%;
		margin-left: -5px;
		border-width: 10px;
		border-style: solid;
		border-color: rgba(185, 28, 28, var(--tw-bg-opacity)) transparent transparent transparent;
	}
</style>
