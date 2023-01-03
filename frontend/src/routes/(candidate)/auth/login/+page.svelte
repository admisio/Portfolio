<script lang="ts">
	import { goto } from '$app/navigation';
	import Submit from '$lib/components/button/Submit.svelte';

	import SchoolBadge from '$lib/components/icons/SchoolBadge.svelte';
	import SplitLayout from '$lib/components/layout/SplitLayout.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';

	let applicationValue = '';

	const redirectToCode = () => {
		// TODO: Validation
		if (applicationValue) {
			goto(`login/${applicationValue}`);
		}
	};
</script>

<SplitLayout>
	<div class="form">
		<SchoolBadge />
		<h1 class="text-sspsBlue mt-8 text-4xl font-semibold">Přihlášení</h1>
		<p class="text-sspsGray my-8 text-center font-light">
			Evidenční číslo je jedinečné číslo přidělené uchazeči, které slouží k jeho identifikaci<br /> a
			přihlášení se do systému.
		</p>
		<div class="w-4/5 lg:w-3/5">
			<TextField bind:value={applicationValue} placeholder="Ev. číslo" type="number" />
		</div>
		<div class="mt-8 w-4/5 lg:w-3/5">
			<Submit on:click={redirectToCode} value="Odeslat" />
		</div>
	</div>
</SplitLayout>

<style lang="postcss">
	.form {
		@apply flex flex-col;
		@apply mx-auto h-full w-[90%];
		@apply items-center justify-center;
	}
</style>
