<script context="module" lang="ts">
	export type GradeBackend = {
		subject: string;
		grade: number;
		semester: Semester;
	};
</script>

<script lang="ts">
	import GradesRow, { type Grade, type Semester } from './GradesRow.svelte';

	let gradesLocal: Array<Grade> = Array.from({ length: 8 }, () => {
		return {
			subject: '',
			semesters: {
				'1/8': undefined,
				'2/8': undefined,
				'1/9': undefined,
				'2/9': undefined
			}
		};
	});

	export let error: string | Array<unknown> = '';

	export let grades: Array<GradeBackend>;

	// Convert local Grade type to expanded GradesBackend type
	const convertGrades = () => {
		// Delay to wait for select to be updated
		setTimeout(() => {
			const gradesTemp: Array<GradeBackend> = [];
			for (let index = 0; index < gradesLocal.length; index++) {
				const grade = gradesLocal[index];
				for (const semester in grade.semesters) {
					const semesterTyped = semester as Semester;
					if (grade.semesters[semesterTyped] && grade.subject) {
						const gradeString = grade.semesters[semesterTyped]!;
						gradesTemp.push({
							subject: grade.subject,
							grade: Number(gradeString),
							semester: semesterTyped
						});
					}
				}
			}
			grades = [...gradesTemp];
		});
	};
</script>

<div class="mx-auto mt-8 flex max-h-[22rem] w-full flex-col overflow-scroll lg:w-4/5">
	<div class="flex text-gray-400">
		<span class="w-1/2 text-center">Známky</span>
		<span class="ml-0.5 w-1/6 text-center">1/8</span>
		<span class="ml-0.5 w-1/6 text-center">2/8</span>
		<span class="ml-0.5 w-1/6 text-center">1/9</span>
		<span class="ml-0.5 w-1/6 text-center">2/9</span>
	</div>
	{#each gradesLocal as _, i}
		<div class="mb-1">
			<GradesRow on:keyup={convertGrades} on:change={convertGrades} bind:grade={gradesLocal[i]} />
		</div>
	{/each}
	<button
		class:isError={error.length > 0}
		class="ml-auto w-24 rounded-full bg-gray-400 p-1 text-xl text-white transition-colors duration-300 hover:bg-gray-500"
		on:click={() => {
			gradesLocal = [
				...gradesLocal,
				{
					subject: '',
					semesters: {
						'1/8': undefined,
						'2/8': undefined,
						'1/9': undefined,
						'2/9': undefined
					}
				}
			];
		}}>+</button
	>
</div>

<style lang="postcss">
	.isError {
		@apply bg-red-500;
	}
</style>
