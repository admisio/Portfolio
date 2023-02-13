<script lang="ts">
	import 'virtual:windi.css';

	import PageTransition from '$lib/components/PageTransition.svelte';

	export let data: { url: string };
</script>

<link rel="preconnect" href="https://fonts.googleapis.com" />
<link rel="preconnect" href="https://fonts.gstatic.com" />
<link
	href="https://fonts.googleapis.com/css2?family=Poppins:wght@200;400;600&display=swap"
	rel="stylesheet"
/>

<PageTransition url={data.url}>
	<slot />
</PageTransition>

<style>
	:global(html) {
		font-family: 'Poppins', sans-serif;
	}
</style>
