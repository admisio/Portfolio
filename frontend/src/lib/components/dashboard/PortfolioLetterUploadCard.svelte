<script lang="ts">
	import type { ApiError } from '$lib/@api';
	import { fetchSubmProgress } from '$lib/stores/portfolio';
	import { apiUploadPortfolioLetter } from '../../@api/candidate';
	import DashboardUploadCard from './DashboardUploadCard.svelte';

	let error: string | null = null;

	const onFileDrop = async (detail: any) => {
		const file = detail.file;
		const callback = detail.callback;
		try {
			await apiUploadPortfolioLetter(file, callback);
			error = null;
		} catch (e) {
			error = (e as ApiError).msg;
		}
		await fetchSubmProgress();
	};
</script>

<DashboardUploadCard
	{error}
	on:filedrop={(e) => onFileDrop(e.detail)}
	title="Portfolio"
	filetype="PDF"
	filesize={10}
	fileType={2}
	placeholder="svoje portfolio"
/>
