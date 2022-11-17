<script lang="ts">
	import lev from '$lib/assets/logo/lev.png';
	import SplitLayout from '$lib/components/layout/SplitLayout.svelte';
</script>

<SplitLayout>
	<div class="form">
		<div
			class="py-3 px-6 md:py-4 md:px-8 rounded-[999px] shadow-lg flex items-center justify-center"
		>
			<img class="object-cover" src={lev} alt="" />
		</div>
		<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Přihlášení</h1>
		<p class="mt-8 font-light text-sspsGray text-center">
			Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
		</p>
		<input
			class="bg-[#f8fafb] mt-8 w-3/5 shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
			type="password"
			placeholder="Ev. číslo"
		/>
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
