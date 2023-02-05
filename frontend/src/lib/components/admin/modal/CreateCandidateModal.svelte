<script lang="ts">
	import type { ApiError } from '$lib/@api';
	import { apiCreateCandidate } from '$lib/@api/admin';
	import SelectField from '$lib/components/select/SelectField.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';
	import type { CreateCandidate, CreateCandidateLogin } from '$lib/stores/candidate';
	import { isPersonalIdNumberValid } from '$lib/utils/personalIdFormat';
	import { createEventDispatcher } from 'svelte';
	import Modal from '../../Modal.svelte';
	import IdField from '../../textfield/IdField.svelte';
	import NumberField from '../../textfield/NumberField.svelte';
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';
	import jsPDF from 'jspdf';
	import 'svg2pdf.js';

	let isOpened = true;

	let applicationId: string = '';
	let citizenship: string = '';
	let personalId: string = '';
	let field: 'GYM' | 'IT' | 'KB' | 'Ev. č. nezadáno';

	let login: CreateCandidateLogin;

	let error: string = '';

	const dispatch = createEventDispatcher();

	$: {
		let prefix = applicationId.slice(0, 3);
		if (Number(prefix) === 101) {
			field = 'GYM';
		} else if (Number(prefix) === 102) {
			field = 'IT';
		} else if (Number(prefix) === 103) {
			field = 'KB';
		} else {
			field = 'Ev. č. nezadáno';
		}
	}

	const createCandidate = async () => {
		if (applicationId.length < 6) {
			toast.push('Ev. číslo musí mít minimálně 6 znaků', {
				theme: {
					'--toastColor': 'mintcream',
					'--toastBackground': '#b91c1c',
					'--toastBarBackground': '#7f1d1d'
				}
			});
			return;
		}
		if (citizenship === 'Česká republika') {
			if (!isPersonalIdNumberValid(personalId)) {
				toast.push('Rodné číslo neodpovídá oficiální specifikaci či datumu narození', {
					theme: {
						'--toastColor': 'mintcream',
						'--toastBackground': '#b91c1c',
						'--toastBarBackground': '#7f1d1d'
					}
				});
				return;
			}
		}
		const data: CreateCandidate = {
			applicationId: Number(applicationId),
			personalIdNumber: personalId
		};
		try {
			login = await apiCreateCandidate(data);
			toast.push(
				`Uživatel ${data.applicationId} s rodným číslem ${data.personalIdNumber} byl vytvořen!`,
				{
					theme: {
						'--toastColor': 'mintcream',
						'--toastBackground': '#047857',
						'--toastBarBackground': '#064e3b'
					}
				}
			);
			dispatch('created');
			error = '';
		} catch (e: unknown) {
			console.error(e);
			error = (e as ApiError).msg;
		}
	};

	const generatePdf = async () => {
		const template = (await import('$lib/assets/pdf/drawing.svg?raw')).default;
		const svg = template
			.replace('${APPLICATION}', login.applicationId.toString())
			.replace('${CODE}', login.password);

		const element = document.getElementById('svg-element')!;
		element.innerHTML = svg;

		const doc = new jsPDF('p', 'mm', [210, 297]);

		await doc.svg(element);

		doc.save('PRIHLASOVACI_UDAJE_' + login.applicationId.toString());
	};

	const close = () => {
		isOpened = false;
		dispatch('close');
	};
</script>

{#if isOpened}
	<SvelteToast />
	<Modal on:close={close}>
		<div class="p-20">
			{#if login}
				<svg width="210mm" height="297mm" class="hidden h-[297mm] w-[210mm]" id="svg-element" />
				<button on:click={generatePdf}>aa</button>
				<h1 class="text-sspsBlue text-3xl font-semibold">Ev. č.: {applicationId}</h1>
				<h1 class="text-sspsBlue text-3xl font-semibold">R. č.: {login.personalIdNumber}</h1>
				<h1 class="text-sspsBlue text-3xl font-semibold">Heslo: {login.password}</h1>
				{#if login.applications.length > 1}
					<h1 class="text-sspsBlue text-3xl font-semibold">
						Slinkovaný s {login.applications.filter((a) => a != applicationId)}
					</h1>
				{/if}
			{:else}
				<h1 class="text-sspsBlue text-3xl font-semibold">Registrace nového uchazeče</h1>
				{#if error}
					<div
						class="relative my-2 rounded border border-red-400 bg-red-100 px-4 py-3 text-red-700"
						role="alert"
					>
						<span class="block sm:inline">{error}</span>
					</div>
				{/if}
				<div>
					<h3 class="my-4">
						Evidenční číslo přihlášky (
						<span class="font-bold">{`Obor: ${field}`}</span>)
					</h3>
					<NumberField bind:value={applicationId} />
				</div>
				<div>
					<h3 class="my-4">Občanství</h3>
					<SelectField
						bind:value={citizenship}
						placeholder="Občanství"
						options={['Česká republika', 'Slovenská republika', 'Ukrajina', 'Jiné']}
					/>
					<h3 class="my-4">Rodné číslo</h3>
				</div>
				<div>
					{#if citizenship === 'Česká republika'}
						<IdField bind:value={personalId} />
					{:else}
						<TextField bind:value={personalId} />
					{/if}
				</div>
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

<style lang="postcss">
</style>
