<script lang="ts">
	import { fetchSubmProgress } from '$lib/stores/portfolio';
	import { apiDeleteCoverLetter, apiUploadCoverLetter } from '$lib/@api/candidate';
	import DashboardUploadCard from './DashboardUploadCard.svelte';

	const onFileDrop = async (detail: any) => {
		const file = detail.file;
		const callback = detail.callback;
		await apiUploadCoverLetter(file, callback);
		await fetchSubmProgress();
	};

	const onDelete = async () => {
		await apiDeleteCoverLetter();
		await fetchSubmProgress();
	};
</script>

<DashboardUploadCard
	on:filedrop={(e) => onFileDrop(e.detail)}
	on:delete={onDelete}
	title="Motivační dopis"
	filetype="PDF"
	filesize={10}
	fileType={1}
	placeholder="svůj motivanční dopis"
/>
