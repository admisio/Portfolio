<script lang="ts">
	import type { Status } from '$lib/stores/portfolio';

	export let status: Status;
	let title: string;

	$: switch (status) {
		case 'submitted':
			title = 'Odeslané';
			break;
		case 'uploaded':
			title = 'Nahráno';
			break;
		case 'missing':
			title = 'Chybí';
			break;
	}
</script>

<!-- make red dot -->
<div class="flex animate-pulse flex-row justify-between div-{status}">
	<span class="h-6 w-6 rounded-full {status}" />
	<!-- <h3 class="ml-8 font-bold text-xl">{title}</h3> -->
</div>

<style lang="postcss">
	span {
		@apply rounded-full p-1;
	}

	.div-submitted {
		@apply animate-none;
	}

	.submitted {
		@apply bg-[#35e000ff];
	}

	.uploaded {
		@apply bg-[#ff8530ff];
		@apply animate-none;
	}

	.missing {
		@apply bg-red-700;
	}
</style>
