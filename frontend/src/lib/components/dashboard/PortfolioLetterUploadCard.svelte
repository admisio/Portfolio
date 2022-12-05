<script lang="ts">
	import { fetchSubmProgress } from '$lib/stores/portfolio';
	import { apiUploadPortfolioLetter } from '../../@api/candidate';
	import DashboardUploadCard from './DashboardUploadCard.svelte';

	const onFileDrop = async (detail: any) => {
		const file = detail.file;
		const callback = detail.callback;
		await apiUploadPortfolioLetter(file, callback);
		await fetchSubmProgress();
	};
</script>

<DashboardUploadCard
	on:filedrop={(e) => onFileDrop(e.detail)}
	title="Portfolio"
	filetype="PDF"
	filesize={10}
	fileType={2}
	placeholder="svoje portfolio"
/>
