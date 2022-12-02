<script lang="ts">
	import type { AxiosProgressEvent } from "axios";
	import { apiUploadPortfolioLetter } from "../../../@api/candidate";
	import DashboardUploadCard from "./DashboardUploadCard.svelte";

    const onFileDrop = async (file: File) => {
        await apiUploadPortfolioLetter(file, (progressEvent: AxiosProgressEvent) => {
            console.log(progressEvent.loaded)
        });
    }

</script>


<DashboardUploadCard
    on:filedrop={e => onFileDrop(e.detail)} 
    title="Portfolio"
    filetype="PDF"
    filesize="10 MB">
</DashboardUploadCard>
