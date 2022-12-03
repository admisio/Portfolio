<script lang="ts">
	import lion from '$lib/assets/logo/lion.png';
	import SplitLayout from '$lib/components/layout/SplitLayout.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';

	import background from '$lib/assets/background2.jpg';
	import Lock from '$lib/components/icons/Lock.svelte';

	let adminIdValue = '';
	let adminPasswordValue = '';
</script>

<SplitLayout backgroundImage={background} backgroundPosition="30%">
	<div class="form">
		<div
			class="py-3 px-6 md:py-4 md:px-8 rounded-[999px] shadow-2xl flex items-center justify-center"
		>
			<img class="object-cover" src={lion} alt="" />
		</div>
		<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Přihlášení</h1>
		<p class="mt-8 font-light text-sspsGray text-center">
			Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
		</p>
		<div class="w-3/5">
			<TextField bind:value={adminIdValue} placeholder="Admin id" type="number" />

			<TextField bind:value={adminPasswordValue} placeholder="Heslo" type="password" icon>
				<div slot="icon" class="flex items-center justify-center">
					<Lock />
				</div>
			</TextField>
		</div>
		<input
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
