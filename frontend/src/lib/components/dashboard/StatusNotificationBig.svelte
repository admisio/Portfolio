<script lang="ts">
	import type { Status } from "$lib/stores/portfolio";

    export let status: Status;

    let title: string;
    $: switch (status) {
        case "submitted":
            title = "Soubory odevzdány!";
            break;
        case "uploaded":
            title = "Soubory nahrány!";
            break;
        case "missing":
            title = "Chybí soubory!";
            break;
    }

</script>
<div class="bg-white w-96 h-16 rounded-full flex mb-8">
    <div class="mt-3 mb-3 ml-3 relative flex flex-row">
        <h2 class="ml-2 ml-12 text-2xl self-center text-sspsBlueDark font-bold">{title}</h2>
        <span class="w-8 h-8 absolute rounded-full self-center {status}" />
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

