<script lang="ts">
	import { fetchSubmProgress } from '$lib/stores/portfolio';
	import { apiDeleteCoverLetter, apiUploadCoverLetter } from '$lib/@api/candidate';
	import DashboardUploadCard from './DashboardUploadCard.svelte';
	import type { ApiError } from '$lib/@api';

	export let compact: boolean = false;
	let error: string | null = null;

	const onFileDrop = async (detail: any) => {
		const file = detail.file;
		const callback = detail.callback;
		try {
			await apiUploadCoverLetter(file, callback);
			error = null;
		} catch (e) {
			error = (e as ApiError).msg;
		}
		await fetchSubmProgress();
	};

	const onDelete = async () => {
		await apiDeleteCoverLetter();
		await fetchSubmProgress();
	};
</script>

<DashboardUploadCard
	{error}
	{compact}
	on:filedrop={(e) => onFileDrop(e.detail)}
	on:delete={onDelete}
	title="Motivační dopis"
	filetype="PDF"
	filesize={10}
	fileType={1}
	placeholder="svůj motivanční dopis"
/>
