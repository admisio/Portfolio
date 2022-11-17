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
		<h1 class="mt-6 text-4xl text-[#406280] font-semibold">Přihlášení</h1>
		<p class="mt-6 font-light text-[#e6e6e6] text-center">
			Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
		</p>
		<input
			class="mt-6 w-3/5 shadow-lg p-3 rounded-lg text-xl"
			type="text"
			placeholder="Evidenční číslo přihlášky"
		/>
		<input
			class="mt-6 w-3/5 shadow-lg p-3 rounded-lg text-xl border border-[#406280] border-2"
			type="password"
			value="............"
		/>
		<input
			class="mt-6 w-3/5 p-3 rounded-lg font-semibold text-xl bg-[#406280] text-white"
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
