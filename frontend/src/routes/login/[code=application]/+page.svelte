<script lang="ts">
	import FullLayout from '$lib/components/layout/FullLayout.svelte';

	import woman from '$lib/assets/woman.png';
</script>

<FullLayout>
	<div class="modal">
		<img class="mx-auto" src={woman} alt="" />
		<div class="flex justify-center  items-center">
			{#each [1, 2, 3, 4] as value}
				<input
					class="mr-2 text-center text-4xl font-semibold text-sspsBlue bg-[#f8fafb] w-16 h-20 shadow-lg p-3 rounded-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
					type="text"
					{value}
				/>
			{/each}
			<span class="mr-2 w-8 h-2 bg-sspsBlue"></span>
			{#each [5, 6, 7, 8] as value}
			<input
				class="mr-2 text-center text-4xl font-semibold text-sspsBlue bg-[#f8fafb] w-16 h-20 shadow-lg p-3 rounded-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
				type="text"
				{value}
			/>
		{/each}
		</div>
		<h3 class="mt-6 text-sspsBlue font-semibold text-xl text-center">
			Zadejte 8místný kód pro aktivaci účtu
		</h3>
		<p class="text-sspsGray text-center">Nevíte si rady? Klikněte <u>zde</u></p>
	</div>
</FullLayout>

<style>
	.modal {
		@apply flex flex-col items-center justify-center;
		@apply mx-auto my-auto;
		@apply w-3/5 h-3/5;
		@apply rounded-3xl;
		@apply bg-white;
	}
</style>
