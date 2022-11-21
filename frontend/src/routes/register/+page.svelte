<script lang="ts">
	import { goto } from '$app/navigation';

	import lev from '$lib/assets/logo/lev.png';
	import SplitLayout from '$lib/components/layout/SplitLayout.svelte';

	let applicationValue = '';

	const redirectToCode = () => {
		// TODO: Validation
		if (applicationValue) {
			goto(`/login/${applicationValue}`);
		}
	};

	let pageIndex = 1;
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
				<input
					class="bg-[#f8fafb] mt-8 w-full shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
					type="text"
					placeholder="Jméno a příjmení"
				/>
				<input
					class="ml-2 md:ml-0 bg-[#f8fafb] mt-8 w-full shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
					type="email"
					placeholder="E-mail"
				/>
			</div>

			<input
				class="w-full bg-[#f8fafb] mt-8 md:w-3/5 shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
				type="tel"
				placeholder="Telefon"
			/>
			<input
				on:click={redirectToCode}
				class="w-full mt-8 md:w-3/5 p-3 rounded-lg font-semibold text-xl transition-colors duration-300 bg-sspsBlue hover:bg-sspsBlueDark text-white"
				type="submit"
				value="Odeslat"
			/>
		{/if}
		{#if pageIndex === 1}
			<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Něco o tobě</h1>
			<p class="block mt-8 font-light text-sspsGray text-center">
				Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
			</p>
			<input
				class="w-full bg-[#f8fafb] mt-8 md:w-3/5 shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
				type="text"
				placeholder="Rodné přijmení"
			/>
			<input
				class="w-full bg-[#f8fafb] mt-8 md:w-3/5 shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
				type="text"
				placeholder="Místo narození"
			/>
			<div class="flex items-center justify-center w-full md:w-3/5">
				<input
					class="bg-[#f8fafb] mt-8 w-full shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
					type="text"
					placeholder="Datum narození"
				/>
				<input
					class="ml-2 bg-[#f8fafb] mt-8 w-full shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
					type="text"
					placeholder="Polaví"
				/>
			</div>
			<input
				on:click={redirectToCode}
				class="w-full mt-8 md:w-3/5 p-3 rounded-lg font-semibold text-xl transition-colors duration-300 bg-sspsBlue hover:bg-sspsBlueDark text-white"
				type="submit"
				value="Odeslat"
			/>
		{/if}
		{#if pageIndex === 2}
			<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Už jen kousek!</h1>
			<p class="block mt-8 font-light text-sspsGray text-center">
				Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
			</p>
			<input
				class="w-full bg-[#f8fafb] mt-8 md:w-3/5 shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
				type="text"
				placeholder="Adresa trvalého bydliště"
			/>

			<input
				class="w-full bg-[#f8fafb] mt-8 md:w-3/5 shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
				type="email"
				placeholder="E-mail zákonného zástupce"
			/>

			<input
				class="w-full bg-[#f8fafb] mt-8 md:w-3/5 shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
				type="tel"
				placeholder="Telefon zákonného zástupce"
			/>
			<input
				on:click={redirectToCode}
				class="w-full mt-8 md:w-3/5 p-3 rounded-lg font-semibold text-xl transition-colors duration-300 bg-sspsBlue hover:bg-sspsBlueDark text-white"
				type="submit"
				value="Odeslat"
			/>
		{/if}
		{#if pageIndex === 3}
			<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Poslední krok</h1>
			<p class="block mt-8 font-light text-sspsGray text-center">
				Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
			</p>
			<input
				class="w-full bg-[#f8fafb] mt-8 md:w-3/5 shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
				type="text"
				placeholder="Občanství"
			/>
			<div class="flex items-center justify-center w-full md:w-3/5">
				<input
					class="bg-[#f8fafb] mt-8 w-full shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
					type="text"
					placeholder="Rodné číslo"
				/>
				<input
					class="ml-2 bg-[#f8fafb] mt-8 w-full shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
					type="text"
					placeholder="Obor"
				/>
			</div>
			<input
				class="w-full bg-[#f8fafb] mt-8 md:w-3/5 shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
				type="text"
				placeholder="Evidenční číslo přihlášky"
			/>
			<input
				on:click={redirectToCode}
				class="w-full mt-8 md:w-3/5 p-3 rounded-lg font-semibold text-xl transition-colors duration-300 bg-sspsBlue hover:bg-sspsBlueDark text-white"
				type="submit"
				value="Odeslat"
			/>
		{/if}

		<div class="mt-8 flex flex-row justify-center">
			<span class="w-3 h-3 rounded-full bg-sspsBlue" />
			<span class="w-3 h-3 ml-2 rounded-full bg-sspsGray" />
			<span class="w-3 h-3 ml-2 rounded-full bg-sspsGray" />
			<span class="w-3 h-3 ml-2 rounded-full bg-sspsGray" />
		</div>
	</div>
</SplitLayout>

<style>
	.form {
		@apply flex flex-col;
		@apply mx-auto w-[90%] h-full;
		@apply items-center justify-center;
	}
</style>
