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
			description = 'Odevzdejte soubory kliknutím zde';
			break;
		case 'missing':
			title = 'Soubory nebyly nahrány!';
			description = 'Nahrajte včechny soubory prosím';
			break;
	}
</script>

<div class="flex flex-col">
	<div class="info flex flex-col {status}">
		<span class="text-white text-xl font-bold">{title}</span>
		<span class="text-white text-md italic">{description}</span>
	</div>
</div>

<style>
	.info {
		@apply flex items-center justify-between;
		@apply py-3 px-6;
		@apply rounded-xl bg-red-700 shadow-md border-red-700;

		@apply hover:cursor-help;
	}

	.submitted {
		@apply bg-green-600;
	}

	.uploaded {
		@apply bg-yellow-600;
		@apply !cursor-pointer;
	}

	.missing {
		@apply bg-orange-700;
	}
</style>
