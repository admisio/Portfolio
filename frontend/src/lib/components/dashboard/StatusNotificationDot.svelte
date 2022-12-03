<script lang="ts">
	import type { Status } from "$lib/stores/portfolio";


    export let status: Status;
    let title: string;

    $: switch (status) {
        case "submitted":
            title = "Odeslané";
            break;
        case "uploaded":
            title = "Nahráno";
            break;
        case "missing":
            title = "Chybí";
            break;
    };
</script>

<!-- make red dot -->
<div class="flex flex-row justify-between">
    <span class="mt-1 w-6 h-6 rounded-full {status}" />
    <!-- <h3 class="ml-8 font-bold text-xl">{title}</h3> -->
</div>

<style>
    span {
		@apply rounded-full p-1;
	}

    .submitted {
        @apply bg-[#35e000ff];
    }

    .uploaded {
        @apply bg-[#ff8530ff];
    }

    .missing {
        @apply bg-red-700;
    }
</style>