<script lang="ts">
	import type { AxiosProgressEvent } from "axios";
	import { fetchSubmProgress } from "../../../stores/portfolio";
	import { apiUploadPortfolioLetter } from "../../../@api/candidate";
	import DashboardUploadCard from "./DashboardUploadCard.svelte";

    const onFileDrop = async (file: File) => {
        await apiUploadPortfolioLetter(file, (progressEvent: AxiosProgressEvent) => {
            console.log(progressEvent.loaded)
        });
        await fetchSubmProgress();
    }

</script>


<DashboardUploadCard
    on:filedrop={e => onFileDrop(e.detail)} 
    title="Portfolio"
    filetype="PDF"
    filesize="10 MB"
    fileType={2}>
</DashboardUploadCard>
