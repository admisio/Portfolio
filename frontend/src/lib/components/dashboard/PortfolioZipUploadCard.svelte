<script lang="ts">
	import type { AxiosProgressEvent } from "axios";
	import { apiUploadPortfolioZip } from "../../../@api/candidate";
	import DashboardUploadCard from "./DashboardUploadCard.svelte";

    const onFileDrop = async (file: File) => {
        await apiUploadPortfolioZip(file, (progressEvent: AxiosProgressEvent) => {
            console.log(progressEvent.loaded)
        });
    }

</script>


<DashboardUploadCard
    on:filedrop={e => onFileDrop(e.detail)} 
    title="Další data"
    filetype="ZIP"
    filesize="100 MB">
</DashboardUploadCard>
