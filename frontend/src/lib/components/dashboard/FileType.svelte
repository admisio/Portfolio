<script lang="ts">
	export let filetype: 'PDF' | 'ZIP';
	export let filesize: string;
</script>

<div>
	<span class="text-sspsBlue text-lg italic">{filetype}</span>
	<span class="text-sspsGray mx-2">/</span>
	<span class="text-sspsBlue text-lg italic">Max {filesize}</span>
</div>

<style lang="postcss">
	div {
		@apply flex items-center justify-between;
		@apply py-1 px-4;
		@apply rounded-xl bg-white shadow-md;

		@apply hover:bg-sspsBlue;
		@apply hover:cursor-pointer;
	}

	div,
	span {
		@apply transition-all duration-300;
	}

	div:hover span {
		@apply text-white;
	}
</style>
