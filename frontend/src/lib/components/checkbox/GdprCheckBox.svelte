<script lang="ts">
	export let value: boolean = false;
</script>

<input type="checkbox" id="gdpr-option" bind:checked={value} class="peer hidden" />
<label
	for="gdpr-option"
	class="inline-flex w-full cursor-pointer items-center justify-between rounded-lg border-2 border-gray-200 bg-white p-5 text-gray-500 hover:bg-gray-50 hover:text-gray-600 peer-checked:border-sspsBlue peer-checked:text-gray-600 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300 dark:peer-checked:text-gray-300"
>
	<div class="block">
        <span class="text-2xl">📜</span>

		<div class="w-full text-lg font-semibold">Souhlas se zpracováním osobních údajů</div>
		<div class="w-full text-sm">Kliknutím vyjaďřujete souhlas se zpracováním osobních údajů</div>
        <div class="w-full text-sm"><a class="underline" href="/gdpr">Zde</a></div>
	</div>
</label>

<style>

</style>
