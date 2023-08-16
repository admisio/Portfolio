<script context="module" lang="ts">
	export type Semester = '1/8' | '2/8' | '1/9' | '2/9';
	export type Grade = {
		subject?: string;
		semesters: {
			[semester in Semester]?: string;
		};
	};
</script>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	export let grade: Grade;
	const SEMESTERS: Semester[] = ['1/8', '2/8', '1/9', '2/9'];

	const deleteRow = () => {
		dispatch('delete');
	};
</script>

<div class="flex">
	<input class="w-1/2" on:keyup bind:value={grade.subject} type="text" />
	{#each SEMESTERS as semester}
		<select class="ml-0.5 w-1/6" on:change bind:value={grade.semesters[semester]} name="">
			<option value="" />
			<option value="1">1</option>
			<option value="2">2</option>
			<option value="3">3</option>
			<option value="4">4</option>
			<option value="5">5</option>
		</select>
	{/each}
	<!-- delete button with 'x' icon -->
	<button on:click={deleteRow} class="ml-0.5 h-6 w-6">
		<svg
			xmlns="http://www.w3.org/2000/svg"
			class="h-6 w-6 stroke-red-700"
			fill="none"
			viewBox="0 0 24 24"
			stroke="currentColor"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M6 18L18 6M6 6l12 12"
			/>
		</svg>
	</button>
</div>

<style lang="postcss">
	input {
		@apply rounded-lg border border-2 bg-[#f8fafb] outline-none;
		@apply w-1/2 w-full px-2;
		@apply transition-colors duration-300;
		--at-apply: "hover:border-sspsBlue";
	}
	select {
		@apply ml-0.5 w-1/6;
		@apply rounded-lg border border-2 bg-[#f8fafb] outline-none;
		@apply transition-colors  duration-300;
		--at-apply: "hover:border-sspsBlue";

		-webkit-appearance: none !important;
		-moz-appearance: none !important;
		appearance: none !important;
	}
</style>
