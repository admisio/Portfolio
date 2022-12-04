<script lang="ts">
	import { apiFetchCandidate, apiListCandidates } from "$lib/@api/admin";
	import type { CandidatePreview } from "$lib/stores/candidate";


    let candidates: [CandidatePreview] = [{}];
    let candidateDetails: { [id: number] : CandidatePreview } = {};
    let currentCandidateId: number = 0;

    getCandidates();

    async function getCandidates() {
        try {
            candidates = await apiListCandidates();
        } catch {
            console.log("error");
        }
    }

    async function getCandidateDetails(id: number) {
        currentCandidateId = id;
        candidateDetails[id] = await apiFetchCandidate(id);
    }
</script>
<div>
    <div class="flex flex-row">
        <div class="list">
            {#each candidates as candidate}
                <div class="candidatePreview flex flex-row">
                    <h1 class="ml-5 text-2xl font-bold self-center">{candidate.applicationId}</h1>
                    <div class="flex flex-col ml-12 mt-4">
                        <h3 class="text-lg font-bold">{candidate.name} {candidate.surname?.toUpperCase()}</h3>
                        <div class="relative">
                            <h3 class="text-lg absolute right-0">Obor: {candidate.study}</h3>
                        </div>
                    </div>
                </div>
            {/each}
        </div>
        <div class="detail">
            <h1>Details here</h1>
        </div>
    </div>
</div>

<style>
    .list {
        @apply h-[100vh] w-96;
        @apply overflow-scroll float-left;
    }

    .detail {
        @apply h-[100vh] w-[calc(100vw-96px)] bg-yellow-300;
        @apply float-left overflow-hidden;
    }
    
    .candidatePreview {
        @apply h-20 w-full bg-gray-200 rounded-xl mt-5;
        @apply hover:cursor-pointer;
    }
</style>