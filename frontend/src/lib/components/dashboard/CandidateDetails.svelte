<script lang="ts">
	import { apiGetCandidatePortfolio, apiResetCandidatePassword } from "$lib/@api/admin";
	import type { CandidateData } from "$lib/stores/candidate";
	import ListElement from "./ListElement.svelte";

    export let id: number;
    export let candidate: CandidateData;

    async function resetCandidatePassword() {
		try {
			const res = await apiResetCandidatePassword(id);
            alert("Nove heslo: " + res.password);
		} catch {
			console.log('error');
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
            console.log(e);
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
        <div class="flex flex-col">
            <button on:click={e => resetCandidatePassword()} class="">Resetovat heslo</button>
            <button on:click={e => downloadPortfolio()} class="mt-40">Stáhnout portfolio</button>
        </div>
    </div>
</div>

<style>
    button {
        @apply bg-sspsBlue hover:bg-sspsBlueDark transition duration-300 rounded-lg;
        @apply text-2xl text-white font-bold px-10 py-4;
    }
</style>
