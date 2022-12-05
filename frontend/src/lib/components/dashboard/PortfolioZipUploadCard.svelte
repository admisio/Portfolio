<script lang="ts">
	import { fetchSubmProgress } from '$lib/stores/portfolio';
	import { apiDeletePortfolioZip, apiUploadPortfolioZip } from '$lib/@api/candidate';
	import DashboardUploadCard from './DashboardUploadCard.svelte';

	const onFileDrop = async (detail: any) => {
		const file = detail.file;
		const callback = detail.callback;
		await apiUploadPortfolioZip(file, callback);
		await fetchSubmProgress();
	};

	const onDelete = async () => {
		await apiDeletePortfolioZip();
		await fetchSubmProgress();
	};
</script>

<DashboardUploadCard
	on:filedrop={(e) => onFileDrop(e.detail)}
	on:delete={onDelete}
	title="Další data"
	filetype="ZIP"
	filesize={100}
	fileType={3}
	placeholder="vaše další soubory ve formátu ZIP"
/>
