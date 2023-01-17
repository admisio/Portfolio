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
	export let grade: Grade;
	const SEMESTERS: Semester[] = ['1/8', '2/8', '1/9', '2/9'];
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
</div>

<style lang="postcss">
	input {
		@apply hover:border-sspsBlue rounded-lg border border-2 bg-[#f8fafb] outline-none;
		@apply w-1/2 w-full px-2;
		@applytransition-colors duration-300;
	}
	select {
		@apply ml-0.5 w-1/6;
		@apply hover:border-sspsBlue rounded-lg border border-2 bg-[#f8fafb] outline-none;
		@apply transition-colors  duration-300;
	}
</style>
