<script lang="ts">
	import Delete from '$lib/components/button/Delete.svelte';
	import type { CandidatePreview } from '$lib/stores/candidate';

	export let candidates: Array<CandidatePreview> = [];

	const formatRustChronoDateTime = (date?: string) => {
		if (!date) return '';
		const [datePart, timePart] = date.split('T');
		const [_, month, day] = datePart.split('-');
		const [hour, minute, second] = timePart.split(':');
		return `${day}. ${month}. ${hour}:${minute}:${Number(second).toFixed(0).padStart(2, '0')}`;
	};
</script>

<div class="flex flex-col">
	<div class="overflow-x-auto sm:-mx-6 lg:-mx-8">
		<div class="inline-block min-w-full py-4 sm:px-6 lg:px-8">
			<div class="overflow-hidden rounded-md border-2  border-[#dfe0e9] ">
				<table class="min-w-full text-center ">
					<thead class="bg-[#f6f4f4] ">
						<tr>
							<th scope="col"> Ev. č. přihlásky </th>
							<th scope="col"> Obor </th>
							<th scope="col"> Rodné číslo </th>
							<th scope="col"> Link </th>
							<th scope="col"> Vytvořeno </th>
							<th scope="col" />
						</tr>
					</thead>
					<tbody>
						{#each candidates as candidate}
							<tr class="border-b bg-white hover:cursor-pointer">
								<td class="hover:text-sspsBlue text-gray-900 hover:font-bold"
									><a
										target="_blank"
										rel="noreferrer"
										href="/admin/candidate/{candidate.applicationId}">{candidate.applicationId}</a
									></td
								>
								<td class="text-gray-900">
									{candidate.fieldOfStudy}
								</td>
								<td class="text-gray-900">
									{candidate.personalIdNumber}
								</td>
								<td class="text-gray-900">
									{candidate.relatedApplications?.filter((a) => a !== candidate.applicationId)}
								</td>
								<td class="text-gray-900">
									{formatRustChronoDateTime(candidate.createdAt)}
								</td>
								<td class="text-sm">
									<Delete id={candidate.applicationId} on:delete value="Odstranit" />
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	</div>
</div>

<style lang="postcss">
	th {
		@apply px-6 py-4 text-sm font-medium text-gray-900;
	}
	td {
		@apply whitespace-nowrap px-6 py-4 text-sm;
	}
</style>
