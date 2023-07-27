<script lang="ts">
	import { apiGetCandidatePortfolio, apiResetCandidatePassword } from '$lib/@api/admin';
	import type { CandidateData } from '$lib/stores/candidate';
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';

	export let id: number;
	export let candidateData: CandidateData;

	let candidate = candidateData.candidate;
	let parents = candidateData.parents;

	async function resetCandidatePassword() {
		try {
			const res = await apiResetCandidatePassword(id);
			alert('Nove heslo: ' + res.password);
		} catch {
			toast.push('Rodné číslo neodpovídá oficiální specifikaci či datumu narození', {
				theme: {
					'--toastColor': 'mintcream',
					'--toastBackground': '#b91c1c',
					'--toastBarBackground': '#7f1d1d'
				}
			});
		}
	}

	async function downloadPortfolio() {
		try {
			const portfolioBlob = await apiGetCandidatePortfolio(id);
			const url = window.URL.createObjectURL(new Blob([portfolioBlob]));
			const link = document.createElement('a');
			link.href = url;
			link.setAttribute('download', 'PORTFOLIO' + '_' + id + '.zip');
			document.body.appendChild(link);
			link.click();
		} catch (e) {
			toast.push('Rodné číslo neodpovídá oficiální specifikaci či datumu narození', {
				theme: {
					'--toastColor': 'mintcream',
					'--toastBackground': '#b91c1c',
					'--toastBarBackground': '#7f1d1d'
				}
			});
		}
	}
</script>

<SvelteToast />
<div class="h-screen w-full flex items-center justify-center">
	<div class="mr-8 max-w-sm">
		<div class="rounded-lg bg-white p-10 shadow-xl">
			<div class="p-2">
				<h3 class="text-center text-2xl font-medium font-semibold leading-8 text-sspsBlue">
					{candidate.name + ' ' + candidate.surname}
				</h3>

				<table class="my-3 text-xs">
					<tbody
						><tr>
							<td class="px-2 py-2 font-semibold text-gray-500">Místo narození</td>
							<td class="px-2 py-2">{candidate.birthplace}</td>
						</tr>
						<tr>
							<td class="px-2 py-2 font-semibold text-gray-500">Datum narození</td>
							<td class="px-2 py-2">{candidate.birthdate}</td>
						</tr>
						<tr>
							<td class="px-2 py-2 font-semibold text-gray-500">Adresa</td>
							<td class="px-2 py-2">{candidate.address}</td>
						</tr>
						<tr>
							<td class="px-2 py-2 font-semibold text-gray-500">Telefon</td>
							<td class="px-2 py-2">{candidate.telephone}</td>
						</tr>
						<tr>
							<td class="px-2 py-2 font-semibold text-gray-500">E-mail</td>
							<td class="px-2 py-2">{candidate.email}</td>
						</tr>
						<tr>
							<td class="px-2 py-2 font-semibold text-gray-500">Rodné číslo</td>
							<td class="px-2 py-2">{candidate.personalIdNumber}</td>
						</tr>
						<tr>
							<td class="px-2 py-2 font-semibold text-gray-500">Pohlaví</td>
							<td class="px-2 py-2">{candidate.sex ?? 'NEUVEDENO'}</td>
						</tr>
					</tbody>
				</table>
			</div>
		</div>
	</div>
	<div class="max-w-sm">
		{#each parents as parent}
			<div class="rounded-lg bg-white p-10 shadow-xl">
				<div class="p-2">
					<h3 class="text-center text-2xl font-medium font-semibold leading-8 text-sspsBlue">
						{parent.name + ' ' + parent.surname}
					</h3>
					<table class="my-3 text-xs">
						<tbody
							><tr>
								<td class="px-2 py-2 font-semibold text-gray-500">Telefon</td>
								<td class="px-2 py-2">{parent.telephone}</td>
							</tr>
							<tr>
								<td class="px-2 py-2 font-semibold text-gray-500">E-mail</td>
								<td class="px-2 py-2">{parent.email}</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
		{/each}

		<div class="my-8">
			<div class="flex flex-col">
				<button on:click={(e) => resetCandidatePassword()}>Resetovat heslo</button>
				<button on:click={(e) => downloadPortfolio()} class="my-8">Stáhnout portfolio</button>
			</div>
		</div>
	</div>
</div>

<style lang="postcss">
	button {
		@apply bg-sspsBlue rounded-lg transition duration-300;
		@apply px-10 py-4 text-2xl font-bold text-white;
		--at-apply: "hover:bg-sspsBlueDark";
	}
</style>
