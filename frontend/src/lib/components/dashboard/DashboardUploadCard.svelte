<script lang="ts">
	import FileType from './FileType.svelte';
	import FileDrop from 'filedrop-svelte';
	import { submissionProgress, UploadStatus, type Status } from '../../../stores/portfolio';
	import { createEventDispatcher } from 'svelte';
	import StatusNotification from './StatusNotification.svelte';

	const dispatch = createEventDispatcher();

	export let title: string;
	export let filetype: 'PDF' | 'ZIP';
	export let filesize: string;
	export let fileType: number = 0;

	let status: Status;

	$: if ($submissionProgress) {
		status = getStatus();
	}

	const getStatus = (): Status => {
		switch ($submissionProgress.status) {
			case UploadStatus.None:
				return 'missing';
			case UploadStatus.Some:
				if (!$submissionProgress.files!.some(code => code === fileType)) {
					return 'uploaded';
				}
			case UploadStatus.All:
				return 'uploaded';
			case UploadStatus.Submitted:
				return 'submitted';
			default:
				return 'missing';
		}
	}

	let dashAnimationProgress = 0;
	let dashAnimationInterval: NodeJS.Timer;

	const dashAnimationStart = () => {
		dashAnimationInterval = setInterval(() => {
			dashAnimationProgress += 1;
			if (dashAnimationProgress == 20) {
				dashAnimationProgress = 0;
			}
		}, 30);
	};

	const dashAnimationStop = () => {
		clearInterval(dashAnimationInterval);
	};

	type Dropped = {
		accepted: Array<File>;
		rejected: Array<File>;
	};
	
	const onFileDrop = (dropped: Dropped) => {
		console.log(dropped);
		if (dropped.accepted.length > 0) {
			dispatch('filedrop', dropped.accepted[0]);
		}
	};
</script>

<div class="card uploadCard">
	<div class="flex flex-col sm:flex-row justify-between sm:items-center">
		<h3>{title}</h3>
		<StatusNotification {status} />
		<div class="mt-1 sm:mt-0">
			<FileType {filetype} {filesize} />
		</div>
	</div>
	<div class="flex-1 mt-6">
		<FileDrop
			multiple={false}
			maxSize={filetype == 'PDF' ? 100_000_000 : 10_000_000}
			accept={filetype == 'PDF' ? 'application/pdf' : 'application/zip'}
			
			on:filedrop={(e) => onFileDrop(e.detail.files)}
			on:filedragenter={dashAnimationStart}
			on:filedragleave={dashAnimationStop}
		>
			<div
				class="drag group"
				on:mouseenter={dashAnimationStart}
				on:mouseleave={dashAnimationStop}
				style={`background-image: url("data:image/svg+xml,%3csvg width='100%25' height='100%25' xmlns='http://www.w3.org/2000/svg'%3e%3crect width='100%25' height='100%25' fill='none' rx='9' ry='9'  stroke-opacity='50%' stroke='%23406280' stroke-width='4' stroke-dasharray='10' stroke-dashoffset='${dashAnimationProgress}' stroke-linecap='square'/%3e%3c/svg%3e");`}
			>
				<span class="text-[#406280]">Sem přetáhněte,</span>
				<span class="text-sspsGray">nebo nahrajte svůj motivační dopis</span>
			</div>
		</FileDrop>
	</div>
</div>

<style>
	:global([type='file']) {
		@apply hidden;
	}
	.card {
		@apply h-full;
		@apply flex flex-col justify-between;

		@apply bg-[#f8fbfc];
		@apply rounded-3xl;
		@apply px-7 py-7;
	}
	.card h3 {
		@apply text-sspsBlue text-2xl xl:text-4xl font-semibold;
	}
	.card span {
		@apply opacity-60 text-sm;
		@apply transition-all duration-300;
	}
	.card .drag {
		@apply transition duration-200;
		@apply min-h-full;
		@apply flex flex-col items-center justify-center;
		border-radius: 9px;

		@apply hover:cursor-pointer;

		/* TODO: Fix this hack */
		@apply p-10 sm:p-20 md:p-0;
	}
	.card .drag:hover span {
		@apply opacity-100;
	}
</style>
