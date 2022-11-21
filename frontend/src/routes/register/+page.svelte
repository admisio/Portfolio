<script lang="ts">
	import { goto } from '$app/navigation';

	import lev from '$lib/assets/logo/lev.png';
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
					<div
						slot="icon"
						class="flex items-center justify-center text-center text-2xl text-sspsBlue pb-1"
					>
						@
					</div>
				</TextField>
			</div>
			<div class="w-full md:w-3/5">
				<TextField type="tel" placeholder="Telefon" icon>
					<div slot="icon" class="flex items-center justify-center">
						<svg
							class="fill-transparent stroke-sspsBlue"
							height="24"
							width="24"
							xmlns="http://www.w3.org/2000/svg"
							><g stroke-linecap="round" stroke-linejoin="round"
								><path
									d="M17.935 19.5C13.535 23.9.1 10.465 4.5 6.065l1.232-1.232a2 2 0 0 1 3.14.405l.535.914a2 2 0 0 1-.071 2.131l-.363.535a1.5 1.5 0 0 0 .182 1.902l2.062 2.063 2.063 2.062a1.5 1.5 0 0 0 1.902.181l.535-.362a2 2 0 0 1 2.131-.07l.914.534a2 2 0 0 1 .404 3.14z"
									stroke-width="2"
								/><path
									d="M13 6.5l4-4m0 0h-2.5m2.5 0V5m4 1.5l-4 4m0 0h2.5m-2.5 0V8"
									stroke-width="1.5"
								/></g
							></svg
						>
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
						<svg height="24" width="24" xmlns="http://www.w3.org/2000/svg"
							><path
								d="M4.586 20.414l-.707.707zm14.828 0l-.707-.707zM19 10v7h2v-7zm-3 10H8v2h8zM5 17v-7H3v7zm3 3c-.971 0-1.599-.002-2.061-.064-.434-.059-.57-.153-.646-.229l-1.414 1.414c.51.51 1.138.709 1.793.797C6.3 22.002 7.085 22 8 22zm-5-3c0 .915-.002 1.701.082 2.328.088.655.287 1.284.797 1.793l1.414-1.414c-.076-.076-.17-.212-.229-.646C5.002 18.6 5 17.971 5 17zm16 0c0 .971-.002 1.599-.064 2.061-.059.434-.153.57-.229.646l1.414 1.414c.51-.51.709-1.138.797-1.793C21.002 18.7 21 17.915 21 17zm-3 5c.915 0 1.701.002 2.328-.082.655-.088 1.284-.287 1.793-.797l-1.414-1.414c-.076.076-.212.17-.646.229-.462.062-1.09.064-2.061.064z"
								class="fill-sspsBlue"
							/><path
								d="M3 11l6.172-6.172c1.333-1.333 2-2 2.828-2s1.495.667 2.828 2L21 11"
								class="stroke-sspsBlue fill-sspsBlue"
								stroke-linecap="round"
								stroke-width="2"
							/><path
								d="M9 17c0-.932 0-1.398.152-1.765a2 2 0 0 1 1.083-1.083C10.602 14 11.068 14 12 14s1.398 0 1.765.152a2 2 0 0 1 1.083 1.083C15 15.602 15 16.068 15 17v4H9zm7-12.5c0-.466 0-.699.076-.883a1 1 0 0 1 .541-.54C16.801 3 17.034 3 17.5 3s.699 0 .883.076a1 1 0 0 1 .54.541c.077.184.077.417.077.883V10l-3-3.5z"
								class="fill-sspsBlue"
							/></svg
						>
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
