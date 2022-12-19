<script lang="ts">
	import { apiCreateCandidate } from '$lib/@api/admin';
	import type { CreateCandidate, CreateCandidateLogin } from '$lib/stores/candidate';
	import { createEventDispatcher } from 'svelte';
	import Modal from '../Modal.svelte';
	import IdField from '../textfield/IdField.svelte';
	import NumberField from '../textfield/NumberField.svelte';
	import { jsPDF } from 'jspdf'
	import 'svg2pdf.js'

	let isOpened = true;

	let applicationId: string = '';
	let personalId: string = '';

	let login: CreateCandidateLogin;

	const dispatch = createEventDispatcher();

	const createCandidate = async () => {
		const data: CreateCandidate = {
			applicationId: Number(applicationId),
			personalIdNumber: personalId
		};
		try {
			login = await apiCreateCandidate(data);
			generatePdf();
			dispatch('created');
		} catch (e) {
			console.log(e);
		}
	};

	const generatePdf = async () => {
		const template = await fetch("/drawing.svg");
		const blob = await template.blob();
		const blobText = await blob.text();
		const svg = blobText.replace("${APPLICATION}", login.applicationId.toString()).replace("${CODE}", login.password);
		
		const element = document.getElementById("svg-element")!;
		element.innerHTML = svg;

		const doc = new jsPDF('p', 'mm', [210, 297]);

		await doc.svg(element);

		doc.save('PRIHLASOVACI_UDAJE_' + login.applicationId.toString());
	}

	const close = () => {
		isOpened = false;
		dispatch('close');
	};
</script>

{#if isOpened}
<Modal on:close={close}>
	<div class="p-20">
			<!-- <div class="fixed -top-32 bg-gray-400"> -->
				<svg width=210mm height=297mm class="w-[210mm] h-[297mm] hidden" id="svg-element"></svg>
			<!-- </div> -->
			{#if login}
				<h1 class="text-sspsBlue text-3xl font-semibold">{applicationId}</h1>
				<h1 class="text-sspsBlue text-3xl font-semibold">{login.password}</h1>
			{:else}
				<h1 class="text-sspsBlue text-3xl font-semibold">Registrace nového uchazeče</h1>
				<h3 class="my-4">Evidenčni číslo přihlášky</h3>
				<NumberField bind:value={applicationId} />
				<h3 class="my-4">Rodné číslo</h3>
				<IdField bind:value={personalId} />
				<input
					on:click={createCandidate}
					class="bg-sspsBlue hover:bg-sspsBlueDark mt-6 w-full rounded-lg p-3 text-xl font-semibold text-white transition-colors duration-300"
					type="submit"
					value="Vytvořit"
				/>
			{/if}
		</div>
	</Modal>
{/if}

<style>
</style>
