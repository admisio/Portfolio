<script context="module" lang="ts">
	export type GradeBackend = {
		subject: string;
		value: number;
		semester: Semester;
	};
</script>

<script lang="ts">
	import GradesRow, { type Grade, type Semester } from './GradesRow.svelte';

	export let error: string | Array<unknown> = '';

	export let grades: Array<GradeBackend>;

	const convertGradeBackendToGrade = (gradesBackend: Array<GradeBackend>) => {
		const grades: Array<Grade> = [];
		for (let index = 0; index < gradesBackend.length; index++) {
			const gradeBackend = gradesBackend[index];
			let grade = grades.find((g) => g.subject === gradeBackend.subject);
			if (!grade) {
				grade = {
					subject: gradeBackend.subject,
					semesters: {}
				};
				grades.push(grade);
			}
			grade.semesters[gradeBackend.semester] = gradeBackend.value.toString();
		}
		return grades;
	};

	let gradesLocal: Array<Grade> =
		grades.length > 0
			? convertGradeBackendToGrade(grades)
			: [
					{ subject: 'Chování', semesters: {} },
					{ subject: 'Český jazyk', semesters: {} },
					{ subject: 'Matematika', semesters: {} },
					{ subject: 'Anglický jazyk', semesters: {} },
					{ subject: 'Chemie', semesters: {} },
					{ subject: 'Fyzika', semesters: {} },
					{ subject: 'Dějepis', semesters: {} },
					{ subject: 'Tělesná výchova', semesters: {} }
			  ];
	/*let gradesLocal: Array<Grade> = Array.from({ length: 8 }, () => {
		return {
			subject: '',
			semesters: {
				'1/8': undefined,
				'2/8': undefined,
				'1/9': undefined,
				'2/9': undefined
			}
		};
	});*/
	// Convert local Grade type to expanded GradesBackend type
	const convertGradeToGradeBackend = () => {
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
							value: Number(gradeString),
							semester: semesterTyped
						});
					}
				}
			}
			grades = [...gradesTemp];
		});
	};
</script>

<div class="mx-auto mt-8 flex pr-6 text-gray-400 lg:w-4/5">
	<span class="w-1/2 text-center">Předmět</span>
	<span class="ml-0.5 w-1/6 text-center">1/8</span>
	<span class="ml-0.5 w-1/6 text-center">2/8</span>
	<span class="ml-0.5 w-1/6 text-center">1/9</span>
	<span class="ml-0.5 w-1/6 text-center">2/9</span>
</div>
<div class="mx-auto max-h-[22rem] w-full flex flex-col overflow-scroll lg:w-4/5">
	<!-- eslint-disable -->
	{#each gradesLocal as __, i}
		<div class="mb-1">
			<GradesRow
				on:keyup={convertGradeToGradeBackend}
				on:change={convertGradeToGradeBackend}
				bind:grade={gradesLocal[i]}
				on:delete={() => {
					grades = grades.filter((grade) => grade.subject !== gradesLocal[i].subject);
					gradesLocal = gradesLocal.filter((_, index) => index !== i);
				}}
			/>
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
