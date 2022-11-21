<script lang="ts">
	export let type: 'text' | 'number' | 'tel' | 'e-mail' = 'text';
	const typeAction = (node: HTMLInputElement) => {
		node.type = type;
	}
	export let placeholder: string = '';
	export let value: string = '';
</script>

<input
	bind:value
	on:click
	on:keydown
	on:keyup
	class="bg-[#f8fafb] mt-8 w-full shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
	use:typeAction
	{placeholder}
/>

<style>
    
</style>