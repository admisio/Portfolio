<script lang="ts">
	import { goto } from '$app/navigation';

	import SchoolBadge from '$lib/components/icons/SchoolBadge.svelte';
	import SplitLayout from '$lib/components/layout/SplitLayout.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';

	let applicationValue = '';

	const redirectToCode = () => {
		// TODO: Validation
		if (applicationValue) {
			goto(`/login/${applicationValue}`);
		}
	};
</script>

<SplitLayout>
	<div class="form">
		<SchoolBadge />
		<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Přihlášení</h1>
		<p class="my-8 font-light text-sspsGray text-center">
			Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
		</p>
		<div class="w-3/5">
			<TextField bind:value={applicationValue} placeholder="Ev. číslo" type="number" />
		</div>
		<input
			on:click={redirectToCode}
			class="mt-8 w-3/5 p-3 rounded-lg font-semibold text-xl transition-colors duration-300 bg-sspsBlue hover:bg-sspsBlueDark text-white"
			type="submit"
			value="Odeslat"
		/>
	</div>
</SplitLayout>

<style>
	.form {
		@apply flex flex-col;
		@apply mx-auto w-[90%] h-full;
		@apply items-center justify-center;
	}
</style>
