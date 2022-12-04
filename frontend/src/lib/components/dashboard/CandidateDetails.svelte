<script lang="ts">
	import { apiResetCandidatePassword } from "$lib/@api/admin";
	import type { CandidateData } from "$lib/stores/candidate";
	import ListElement from "./ListElement.svelte";

    export let candidate: CandidateData;

    async function resetCandidatePassword(id: number) {
		try {
			await apiResetCandidatePassword(id);
		} catch {
			console.log('error');
		}
	}
</script>

<div class="flex flex-row">
    <div class="w-96">
        <li>
            <ListElement label="Jméno" content={candidate.name}></ListElement>
            <ListElement label="Příjmení" content={candidate.surname}></ListElement>
            <ListElement label="Místo narození" content={candidate.birthplace}></ListElement>
            <ListElement label="Datum narození" content={candidate.birthdate}></ListElement>
            <ListElement label="Adresa" content={candidate.address}></ListElement>
            <ListElement label="Telefon" content={candidate.telephone}></ListElement>
            <ListElement label="Email" content={candidate.email}></ListElement>
            <ListElement label="Obor" content={candidate.study}></ListElement>
            <ListElement label="Rodné číslo" content={candidate.personalIdNumber}></ListElement>

            <ListElement label="Jméno rodiče" content={candidate.parentName}></ListElement>
            <ListElement label="Příjmení rodiče" content={candidate.parentSurname}></ListElement>
            <ListElement label="Telefon rodiče" content={candidate.parentTelephone}></ListElement>
            <ListElement label="Email rodiče" content={candidate.parentEmail}></ListElement>
        </li>
    </div>
    <div class="ml-20">
        <div class="bg-sspsBlue hover:bg-sspsBlueDark transition duration-300 rounded-lg px-10 py-4">
            <button class="text-2xl text-white font-bold">Resetovat heslo</button>
        </div>
    </div>
</div>

<style>
</style>
