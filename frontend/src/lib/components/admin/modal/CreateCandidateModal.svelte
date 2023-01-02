<script lang="ts">
	import type { ApiError } from '$lib/@api';
	import { apiCreateCandidate } from '$lib/@api/admin';
	import type { CreateCandidate, CreateCandidateLogin } from '$lib/stores/candidate';
	import { createEventDispatcher } from 'svelte';
	import Modal from '../../Modal.svelte';
	import IdField from '../../textfield/IdField.svelte';
	import NumberField from '../../textfield/NumberField.svelte';

	let isOpened = true;

	let applicationId: string = '';
	let personalId: string = '';

	let login: CreateCandidateLogin;

	let error: string = '';

	const dispatch = createEventDispatcher();

	const createCandidate = async () => {
		const data: CreateCandidate = {
			applicationId: Number(applicationId),
			personalIdNumber: personalId
		};
		try {
			login = await apiCreateCandidate(data);
			dispatch('created');
		} catch (e: unknown) {
			console.error(e);
			error = (e as ApiError).msg;
		}
	};

	const close = () => {
		isOpened = false;
		dispatch('close');
	};
</script>

{#if isOpened}
	<Modal on:close={close}>
		<div class="p-20">
			{#if login}
				<h1 class="text-sspsBlue text-3xl font-semibold">{applicationId}</h1>
				<h1 class="text-sspsBlue text-3xl font-semibold">{login.password}</h1>
			{:else}
				<h1 class="text-sspsBlue text-3xl font-semibold">Registrace nového uchazeče</h1>
				{#if error}
					<div class="my-2 bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
						<span class="block sm:inline">{error}</span>
					</div>
				{/if}
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

<style lang="postcss">
</style>
