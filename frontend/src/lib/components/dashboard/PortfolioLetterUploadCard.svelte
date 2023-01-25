<script lang="ts">
	import LL from '$i18n/i18n-svelte';

	import type { ApiError } from '$lib/@api';
	import { fetchSubmProgress } from '$lib/stores/portfolio';
	import { apiDeletePortfolioLetter, apiUploadPortfolioLetter } from '../../@api/candidate';
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

	const onDelete = async () => {
		await apiDeletePortfolioLetter();
		await fetchSubmProgress();
	};
</script>

<DashboardUploadCard
	{error}
	on:filedrop={(e) => onFileDrop(e.detail)}
	on:delete={onDelete}
	title={$LL.components.dashboard.portfolioLetterUploadCard.title()}
	filetype="PDF"
	filesize={10}
	fileType={2}
	placeholder={$LL.components.dashboard.portfolioLetterUploadCard.placeholder()}
/>
