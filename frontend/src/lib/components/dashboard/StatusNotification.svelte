<script lang="ts">
    export let type: "submitted" | "uploaded" | "missing";
    let title: string;
    switch (type) {
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
<div class="flex flex-col justify-between">
    <span class="mt-1 w-5 h-5 rounded-full {type}" />
    <h3 class="ml-8 font-bold text-xl">{title}</h3>
</div>

<style>
    span {
		@apply absolute rounded-full p-1;
	}

    .submitted {
        @apply bg-green-700;
    }

    .uploaded {
        @apply bg-yellow-700;
    }

    .missing {
        @apply bg-red-700;
    }
</style>