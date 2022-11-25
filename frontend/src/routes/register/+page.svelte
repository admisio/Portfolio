<script lang="ts">
	import { goto } from '$app/navigation';

	import lev from '$lib/assets/logo/lev.png';
	import Email from '$lib/components/icons/Email.svelte';
	import Home from '$lib/components/icons/Home.svelte';
	import Telephone from '$lib/components/icons/Telephone.svelte';
	import SplitLayout from '$lib/components/layout/SplitLayout.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';

	let applicationValue = '';

	const redirectToCode = () => {
		// TODO: Validation
		if (applicationValue) {
			goto(`/login/${applicationValue}`);
		}
	};

	const pageCount = 3;
	let pageIndex = 0;
</script>

<SplitLayout>
	<div class="form">
		<div
			class="py-3 px-6 md:py-4 md:px-8 rounded-[999px] shadow-2xl flex items-center justify-center"
		>
			<img class="object-cover" src={lev} alt="" />
		</div>
		{#if pageIndex === 0}
			<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Registrace</h1>
			<p class="block mt-8 font-light text-sspsGray text-center">
				Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
			</p>
			<div class="flex md:flex-col items-center justify-center w-full md:w-3/5">
				<TextField type="text" placeholder="Jméno a příjmení" />
				<TextField type="e-mail" placeholder="Email" icon>
					<div slot="icon" class="flex items-center justify-center">
						<Email />
					</div>
				</TextField>
			</div>
			<div class="w-full md:w-3/5">
				<TextField type="tel" placeholder="Telefon" icon>
					<div slot="icon" class="flex items-center justify-center">
						<Telephone />
					</div>
				</TextField>
			</div>
		{/if}
		{#if pageIndex === 1}
			<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Něco o tobě</h1>
			<p class="block mt-8 font-light text-sspsGray text-center">
				Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
			</p>
			<div class="flex flex-col w-full md:w-3/5">
				<TextField type="text" placeholder="Rodné příjmení" />
				<TextField type="text" placeholder="Místo narození" icon>
					<div slot="icon" class="flex items-center justify-center">
						<Home />
					</div>
				</TextField>
			</div>

			<div class="flex items-center justify-center w-full md:w-3/5">
				<TextField type="text" placeholder="Datum narození" />
				<TextField type="text" placeholder="Pohlaví" />
			</div>
		{/if}
		{#if pageIndex === 2}
			<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Už jen kousek!</h1>
			<p class="block mt-8 font-light text-sspsGray text-center">
				Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
			</p>
			<div class="flex flex-col w-full md:w-3/5">
				<TextField type="text" placeholder="Adresa trvalého bydliště" />
				<TextField type="e-mail" placeholder="E-mail zákonného zástupce" />
				<TextField type="tel" placeholder="Telefon zákonného zástupce" />
			</div>
		{/if}
		{#if pageIndex === 3}
			<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Poslední krok</h1>
			<p class="block mt-8 font-light text-sspsGray text-center">
				Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
			</p>
			<div class="flex flex-col w-full md:w-3/5">
				<TextField type="text" placeholder="Občanství" icon>
					<div slot="icon">ssj</div>
				</TextField>
			</div>
			<div class="flex items-center justify-center w-full md:w-3/5">
				<TextField type="text" placeholder="Rodné číslo" />
				<TextField type="text" placeholder="Obor" />
			</div>
			<div class="flex flex-col w-full md:w-3/5">
				<TextField type="text" placeholder="Evidenční číslo přihlášky" />
			</div>
		{/if}
		<input
			on:click={() => {
				if (pageIndex === pageCount) {
					//TODO: Submit
				} else {
					pageIndex++;
				}
			}}
			class="w-full mt-8 md:w-3/5 p-3 rounded-lg font-semibold text-xl transition-colors duration-300 bg-sspsBlue hover:bg-sspsBlueDark text-white"
			type="submit"
			value={pageIndex === pageCount ? 'Odeslat' : 'Pokračovat'}
		/>

		<div class="mt-8 flex flex-row justify-center">
			{#each Array(pageCount + 1) as _, i}
				<span class:dotActive={i === pageIndex} class="ml-2 w-3 h-3 rounded-full bg-sspsGray" />
			{/each}
		</div>
	</div>
</SplitLayout>

<style>
	.form {
		@apply flex flex-col;
		@apply mx-auto w-[90%] h-full;
		@apply items-center justify-center;
	}
	.dotActive {
		@apply bg-sspsBlue;
	}
</style>
