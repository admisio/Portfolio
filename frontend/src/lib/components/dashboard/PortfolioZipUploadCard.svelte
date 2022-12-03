<script lang="ts">
	import { fetchSubmProgress } from "$lib/stores/portfolio";
	import { apiUploadPortfolioZip } from "$lib/@api/candidate";
	import DashboardUploadCard from "./DashboardUploadCard.svelte";

    const onFileDrop = async (detail: any) => {
        const file = detail.file;
        const callback = detail.callback;
        await apiUploadPortfolioZip(file, callback);
        await fetchSubmProgress();
    }

</script>


<DashboardUploadCard
    on:filedrop={e => onFileDrop(e.detail)} 
    title="Další data"
    filetype="ZIP"
    filesize={100}
    fileType={3}>
</DashboardUploadCard>
