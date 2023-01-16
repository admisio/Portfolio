<script lang="ts">
	export let linkOk: boolean = false;
	export let linkError: boolean = false;
	export let applications: Array<number>;

	let title1 = `Ano, podával/a jsem dvě přihlášky na dva obory SSPŠaG (${applications[0]} a ${applications[1]})`;
	let title2 = `Ne, přihlášku na SSPŠaG jsem podával/a jen jednu (${applications[0]})`;

	if (applications.length === 1) {
		title1 = `Ano, přihlášku na SSPŠaG jsem podával/a jen jednu (${applications[0]})`;
		title2 = `Ne, přihlášku na SSPŠaG jsem podával více přihlášek`;
	}

	$: console.log(linkOk, linkError);

	export let error: string = '';

	const switchSelection = (id: number) => {
		if (id === 0) {
			linkOk = true;
			linkError = false;
		} else {
			linkOk = false;
			linkError = true;
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
		checked={linkOk}
		class="peer hidden"
	/>
	<label for="linkOk" class="peer-checked:border-sspsBlue peer-checked:text-gray-600" class:error>
		<div class="block">
			<span class="text-2xl">📜</span>

			<div class="w-full text-lg font-semibold">
				{title1}
			</div>
			<div class="w-full text-sm">Vše je v pořádku</div>
		</div>
	</label>
</div>
<div class="mt-2">
	<input
		on:click={(_) => switchSelection(1)}
		on:change
		type="checkbox"
		id="linkError"
		checked={linkError}
		class="peer hidden"
	/>
	<label for="linkError" class="peer-checked:border-sspsBlue peer-checked:text-gray-600">
		<div class="block">
			<span class="text-2xl">📜</span>

			<div class="w-full text-lg font-semibold">
				{title2}
			</div>
			<div class="w-full text-sm">Co se děje?</div>
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
