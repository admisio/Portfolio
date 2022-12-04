<script lang="ts">
	import CandidateDetails from '$lib/components/list/CandidateDetails.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<CandidateDetails id={data.id} candidate={data.candidate} />

<style>
</style>
