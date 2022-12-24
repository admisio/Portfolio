<script lang="ts">
	export let value: boolean = false;

	export let error: string = '';
</script>

<input class:error on:change type="checkbox" id="gdpr" bind:checked={value} class="peer hidden" />
<label for="gdpr" class="peer-checked:border-sspsBlue peer-checked:text-gray-600" class:error>
	<div class="block">
		<span class="text-2xl">📜</span>

		<div class="w-full text-lg font-semibold">Souhlas se zpracováním osobních údajů</div>
		<div class="w-full text-sm">Kliknutím vyjaďřujete souhlas se zpracováním osobních údajů</div>
		<div class="w-full text-sm"><a class="underline" href="/gdpr">Zde</a></div>
	</div>
</label>

<style lang="postcss">
	label {
		@apply inline-flex  w-full items-center justify-between;
		@apply cursor-pointer;
		@apply bg-white p-5 text-gray-500;
		@apply hover:bg-gray-50 hover:text-gray-600;
		@apply rounded-lg border-2 border-gray-200;
	}
	.error {
		@apply border-red-700;
	}
</style>
