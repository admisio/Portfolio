<script lang="ts">
	import CandidateDetails from '$lib/components/admin/list/CandidateDetails.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<CandidateDetails id={data.id} candidateData={data.candidateData} />

<style>
</style>
