<script lang="ts">
	import LL from '$i18n/i18n-svelte';

	export let personalIdOk: boolean = false;
	export let personalIdErr: boolean = false;
	export let personalIdNumber: string;
	let titleOk = $LL.components.checkbox.personalIdConfirmCheckBox.titleOk({
		personalId: personalIdNumber,
	});
	let titleErr = $LL.components.checkbox.personalIdConfirmCheckBox.titleErr({
		personalId: personalIdNumber,
	});

	export let error: string = '';

	const switchSelection = (id: number) => {
		if (id === 0) {
			personalIdOk = true;
			personalIdErr = false;
		} else {
			personalIdOk = false;
			personalIdErr = true;
		}
	};
</script>

<div>
	<input
		on:click={(_) => switchSelection(0)}
		class:error
		on:change
		type="checkbox"
		id="linkOk"
		checked={personalIdOk}
		class="peer hidden"
	/>
	<label for="linkOk" class="peer-checked:border-sspsBlue peer-checked:text-gray-600" class:error>
		<div class="block">
			<span class="text-2xl">📜</span>

			<div class="w-full text-lg font-semibold">
				{titleOk}
			</div>
			<div class="w-full text-sm">{$LL.components.checkbox.personalIdConfirmCheckBox.ok()}</div>
		</div>
	</label>
</div>
<div class="mt-2">
	<input
		on:click={(_) => switchSelection(1)}
		on:change
		type="checkbox"
		id="linkError"
		checked={personalIdErr}
		class="peer hidden"
	/>
	<label for="linkError" class="peer-checked:border-sspsBlue peer-checked:text-gray-600">
		<div class="block">
			<span class="text-2xl">📜</span>

			<div class="w-full text-lg font-semibold">
				{titleErr}
			</div>
			<div class="w-full text-sm">{$LL.components.checkbox.personalIdConfirmCheckBox.whatHappened()}</div>
		</div>
	</label>
</div>

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
