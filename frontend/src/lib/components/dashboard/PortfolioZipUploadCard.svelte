<script lang="ts">
	import LL from '$i18n/i18n-svelte';

	import { fetchSubmProgress } from '$lib/stores/portfolio';
	import { apiDeletePortfolioZip, apiUploadPortfolioZip } from '$lib/@api/candidate';
	import DashboardUploadCard from './DashboardUploadCard.svelte';
	import type { ApiError } from '$lib/@api';

	let error: string | null = null;

	const onFileDrop = async (detail: any) => {
		const file = detail.file;
		const callback = detail.callback;
		try {
			await apiUploadPortfolioZip(file, callback);
			error = null;
		} catch (e) {
			error = (e as ApiError).msg;
		}
		await fetchSubmProgress();
	};

	const onDelete = async () => {
		await apiDeletePortfolioZip();
		await fetchSubmProgress();
	};
</script>

<DashboardUploadCard
	{error}
	on:filedrop={(e) => onFileDrop(e.detail)}
	on:delete={onDelete}
	title={$LL.components.dashboard.portfolioZipUploadCard.title()}
	filetype="ZIP"
	filesize={100}
	fileType={3}
	placeholder={$LL.components.dashboard.portfolioZipUploadCard.placeholder()}
/>
