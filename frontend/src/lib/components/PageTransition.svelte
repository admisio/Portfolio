<script>
	import { fly } from 'svelte/transition';
	export let url = '';
</script>

{#key url}
	<div
		in:fly={{ x: -5, duration: 500, delay: 500 }}
		out:fly={{ x: 5, duration: 500 }}
		class="absolute h-full w-full bg-inherit"
	>
		<slot />
	</div>
{/key}
