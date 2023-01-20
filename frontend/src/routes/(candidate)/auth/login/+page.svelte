<script lang="ts">
	import LL from '$i18n/i18n-svelte';
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
		<h1 class="text-sspsBlue mt-8 text-4xl font-semibold">{$LL.candidate.auth.login.title()}</h1>
		<p class="text-sspsGray my-8 text-center font-light">
			{$LL.candidate.auth.login.description()}
		</p>
		<div class="w-4/5 lg:w-3/5">
			<TextField bind:value={applicationValue} placeholder={$LL.input.evidenceNumber()} type="number" />
		</div>
		<div class="mt-8 w-4/5 lg:w-3/5">
			<Submit on:click={redirectToCode} value={$LL.input.submit()} />
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
