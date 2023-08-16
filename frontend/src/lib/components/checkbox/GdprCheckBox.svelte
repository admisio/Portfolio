<script lang="ts">
	import LL from '$i18n/i18n-svelte';

	export let value = false;

	export let error = '';
</script>

<input class:error on:change type="checkbox" id="gdpr" bind:checked={value} class="peer hidden" />
<label for="gdpr" class="peer-checked:border-sspsBlue peer-checked:text-gray-600" class:error>
	<div class="block">
		<span class="text-2xl">📜</span>

		<div class="w-full text-lg font-semibold">{$LL.components.checkbox.gdprCheckBox.title()}</div>
		<div class="w-full text-sm">{$LL.components.checkbox.gdprCheckBox.description()}</div>
		<div class="w-full text-sm">
			<a class="underline" target="_blank" rel="noreferrer" href="https://www.ssps.cz/kontakt/gdpr/"
				>{$LL.components.checkbox.gdprCheckBox.here()}</a
			>
		</div>
	</div>
</label>

<style lang="postcss">
	label {
		@apply inline-flex  w-full items-center justify-between;
		@apply cursor-pointer;
		@apply bg-white p-5 text-gray-500;
		@apply rounded-lg border-2 border-gray-200;
		--at-apply: "hover:bg-gray-50 hover:text-gray-600";
	}
	.error {
		@apply border-red-700;
	}
</style>
