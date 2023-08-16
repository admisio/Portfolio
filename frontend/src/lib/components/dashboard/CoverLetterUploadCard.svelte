<script lang="ts">
	import LL from '$i18n/i18n-svelte';

	import { fetchSubmProgress } from '$lib/stores/portfolio';
	import { apiDeleteCoverLetter, apiUploadCoverLetter } from '$lib/@api/candidate';
	import DashboardUploadCard from './DashboardUploadCard.svelte';
	import type { ApiError } from '$lib/@api';
	import type { AxiosProgressEvent } from 'axios';

	let error: string | null = null;

	type Detail = {
		file: File;
		callback: (progress: AxiosProgressEvent) => void;
	};

	const onFileDrop = async (detail: Detail) => {
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
	on:filedrop={(e) => onFileDrop(e.detail)}
	on:delete={onDelete}
	title={$LL.components.dashboard.coverLetterUploadCard.title()}
	filetype="PDF"
	filesize={10}
	fileType={1}
	placeholder={$LL.components.dashboard.coverLetterUploadCard.placeholder()}
/>
