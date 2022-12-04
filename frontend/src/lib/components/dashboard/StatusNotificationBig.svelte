<script lang="ts">
	import type { Status } from '$lib/stores/portfolio';

	export let status: Status;

	let title: string;
	let description: string;
	$: switch (status) {
		case 'submitted':
			title = 'Soubory odevzdány!';
			description = 'Vaše soubory byly odevzdány';
			break;
		case 'uploaded':
			title = 'Soubory nebyly odevzdány!';
			description = 'Odevzdejte soubory prosím';
			break;
		case 'missing':
			title = 'Soubory nebyly nahrány!';
			description = 'Nahrajte včechny soubory prosím';
			break;
	}
</script>

<div class="flex flex-col">
	<div class="info flex flex-col {status}">
		<span class="text-white text-2xl font-bold">{title}</span>
		<span class="text-white text-md italic">{description}</span>
	</div>
</div>

<style>
	.info {
		@apply flex items-center justify-between;
		@apply py-3 px-8;
		@apply rounded-xl bg-red-700 shadow-md border-red-700;

		@apply hover:cursor-help;
	}

	.submitted {
		@apply bg-green-600;
	}

	.uploaded {
		@apply bg-yellow-600;
	}

	.missing {
		@apply bg-[#ff3030ff];
	}
</style>
