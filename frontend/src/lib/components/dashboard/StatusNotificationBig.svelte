<script lang="ts">
	import type { Status } from "$lib/stores/portfolio";
    import StatusNotificationDot from "./StatusNotificationDot.svelte";

    export let status: Status;

    let title: string;
    switch (status) {
        case "submitted":
            title = "Soubory odeslány!";
            break;
        case "uploaded":
            title = "Soubory nahrány!";
            break;
        case "missing":
            title = "Chybí soubory!";
            break;
    }

</script>
<div class="bg-white rounded-full flex mb-8">
    <div class="mt-3 mb-3 ml-3 flex flex-row">
        <h2 class="ml-2 text-2xl text-sspsBlueDark font-bold">{title}</h2>
        <span class="ml-32  w-8 h-8 rounded-full self-center {status}" />
        <!-- <StatusNotificationDot {status} /> -->
    </div>
</div>

<style>
    .submitted {
        @apply bg-[#35e000ff];
    }

    .uploaded {
        @apply bg-[#ff8530ff];
    }

    .missing {
        @apply bg-[#ff3030ff];
    }
</style>

