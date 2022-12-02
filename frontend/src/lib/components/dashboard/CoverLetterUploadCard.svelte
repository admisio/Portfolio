<script lang="ts">
	import type { AxiosProgressEvent } from "axios";
	import { apiUploadCoverLetter } from "../../../@api/candidate";
	import DashboardUploadCard from "./DashboardUploadCard.svelte";

    const onFileDrop = async (file: File) => {
        await apiUploadCoverLetter(file, (progressEvent: AxiosProgressEvent) => {
            console.log(progressEvent.progress)
        });
    }

</script>


<DashboardUploadCard 
    on:filedrop={e => onFileDrop(e.detail)}
    title="Motivační dopis"
    filetype="PDF"
    filesize="10 MB">
</DashboardUploadCard>
