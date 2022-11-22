<script lang="ts">
	export let filetype: 'PDF' | 'ZIP';
	export let filesize: string;
</script>

<div>
	<span class="text-sspsBlue italic text-sm">{filetype}</span>
	<span class="mx-2 text-sspsGray">/</span>
	<span class="text-sspsBlue italic text-sm">Max {filesize}</span>
</div>

<style>
	div {
		@apply flex items-center justify-between;
		@apply py-1 px-4;
		@apply bg-white rounded-xl shadow-md;

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
