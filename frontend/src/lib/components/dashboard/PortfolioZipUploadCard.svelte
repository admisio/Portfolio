<script lang="ts">
	import type { AxiosProgressEvent } from "axios";
	import { fetchSubmProgress } from "../../../stores/portfolio";
	import { apiUploadPortfolioZip } from "../../../@api/candidate";
	import DashboardUploadCard from "./DashboardUploadCard.svelte";

    const onFileDrop = async (file: File) => {
        await apiUploadPortfolioZip(file, (progressEvent: AxiosProgressEvent) => {
            console.log(progressEvent.loaded)
        });
        await fetchSubmProgress();
    }

</script>


<DashboardUploadCard
    on:filedrop={e => onFileDrop(e.detail)} 
    title="Další data"
    filetype="ZIP"
    filesize="100 MB"
    fileType={3}>
</DashboardUploadCard>
