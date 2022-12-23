<script lang="ts">
	export let value: string;
</script>

<input on:click type="submit" {value} />

<style>
	input {
		@apply bg-sspsBlue hover:bg-sspsBlueDark 
        @apply @apply rounded-lg p-3 text-xl font-semibold 
        text-white transition-colors duration-300;
		@apply w-full;
	}
</style>
