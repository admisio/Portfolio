<script lang="ts">
	export let filetype: 'PDF' | 'ZIP';
	export let filesize: string;
</script>

<div>
	<span class="text-lg italic text-sspsBlue">{filetype}</span>
	<span class="mx-2 text-sspsGray">/</span>
	<span class="text-lg italic text-sspsBlue">Max {filesize}</span>
</div>

<style lang="postcss">
	div {
		@apply flex items-center justify-between;
		@apply px-4 py-1;
		@apply rounded-xl bg-white shadow-md;

		--at-apply: "hover:bg-sspsBlue hover:cursor-pointer": 
	}

	div,
	span {
		@apply transition-all duration-300;
	}

	div:hover span {
		@apply text-white;
	}
</style>
