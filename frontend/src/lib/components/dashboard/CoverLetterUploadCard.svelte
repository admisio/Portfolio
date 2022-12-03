<script lang="ts">
	import { fetchSubmProgress } from "$lib/stores/portfolio";
	import { apiUploadCoverLetter } from "$lib/@api/candidate";
	import DashboardUploadCard from "./DashboardUploadCard.svelte";

    const onFileDrop = async (detail: any) => {
        const file = detail.file;
        const callback = detail.callback;
        await apiUploadCoverLetter(file, callback);
        await fetchSubmProgress();
    }

</script>


<DashboardUploadCard 
    on:filedrop={e => onFileDrop(e.detail)}
    title="Motivační dopis"
    filetype="PDF"
    filesize={10}
    fileType={1}
    >
</DashboardUploadCard>
