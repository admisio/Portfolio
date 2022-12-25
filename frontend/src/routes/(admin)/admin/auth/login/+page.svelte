<script lang="ts">
	import lion from '$lib/assets/logo/lion.png';
	import SplitLayout from '$lib/components/layout/SplitLayout.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';

	import background from '$lib/assets/background2.jpg';
	import Lock from '$lib/components/icons/Lock.svelte';
	import { apiLogin } from '$lib/@api/admin';
	import { goto } from '$app/navigation';
	import Submit from '$lib/components/button/Submit.svelte';
	import PasswordField from '$lib/components/textfield/PasswordField.svelte';

	let adminIdValue = '';
	let adminPasswordValue = '';

	const login = async () => {
		try {
			await apiLogin({ adminId: Number(adminIdValue), password: adminPasswordValue });
			goto('/admin/dashboard');
		} catch (e) {
			console.log(e);
		}
	};
</script>

<SplitLayout backgroundImage={background} backgroundPosition="30%">
	<div class="form">
		<div
			class="flex items-center justify-center rounded-[999px] py-3 px-6 shadow-2xl md:py-4 md:px-8"
		>
			<img class="object-cover" src={lion} alt="" />
		</div>
		<h1 class="text-sspsBlue mt-8 text-4xl font-semibold">Přihlášení</h1>
		<p class="text-sspsGray mt-8 text-center font-light">
			Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
		</p>
		<div class="mt-8 flex w-3/5 flex-col">
			<span>
				<TextField bind:value={adminIdValue} placeholder="Admin id" type="number" />
			</span>
			<span class="mt-8">
				<PasswordField bind:value={adminPasswordValue} placeholder="Heslo" />
			</span>
		</div>
		<div class="mt-8  w-3/5">
			<Submit value="Odeslat" on:click={login} />
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
