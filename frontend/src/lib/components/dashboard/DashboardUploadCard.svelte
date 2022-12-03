<script lang="ts">
	import FileType from './FileType.svelte';
	import FileDrop from 'filedrop-svelte';
	import { submissionProgress, UploadStatus, type Status } from '$lib/stores/portfolio';
	import { createEventDispatcher } from 'svelte';
	import ProgressBar from './ProgressBar.svelte';
	import type { AxiosProgressEvent } from 'axios';
	import StatusNotificationDot from './StatusNotificationDot.svelte';

	import documentIcon from '$lib/assets/document.png';

	const dispatch = createEventDispatcher();

	export let title: string;
	export let filetype: 'PDF' | 'ZIP';
	export let filesize: string;
	export let fileType: number;

	let fileDropped: boolean = false;
	let progress: number = 1;
	let bytesTotal: number = 0;

	let status: Status;

	$: if ($submissionProgress) {
		status = getStatus();
		// console.log('type' + fileType + ' status: ' + status);
		fileDropped = status === 'uploaded' || status === 'submitted';
	}

	const getStatus = (): Status => {
		console.log($submissionProgress);
		switch ($submissionProgress.status) {
			case UploadStatus.None:
				return 'missing';
			case UploadStatus.Some:
				if ($submissionProgress.files!.some(code => code === fileType)) {
					return 'uploaded';
				}
				return 'missing';
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
			fileDropped = true;
			const file = dropped.accepted[0];
			// send the request in outer component
			dispatch('filedrop', {
				file: file,
				callback: (progressEvent: AxiosProgressEvent) => {
					console.log(progressEvent.bytes);
					progress = progressEvent.progress!;
					bytesTotal = progressEvent.total ?? 0;
				}
			});
		}
	};
</script>

<div class="relative card uploadCard">
	<div class="flex flex-col sm:flex-row justify-between sm:items-center">
		<h3 class="">{title}</h3>
		<div class="mt-1 sm:mt-0">
			<FileType {filetype} {filesize} />
		</div>
		<div class="absolute px-7 right-0 top-4">
			<StatusNotificationDot {status} />
		</div>
	</div>
	{#if fileDropped}	
		<div class="flex content-around justify-between items-center mb-5 ml-5 mr-5">
			<img src={documentIcon} alt="">
			<svg class="h-25" viewBox="0 0 2 40" xmlns="http://www.w3.org/2000/svg"><line x1="0" y="0" x2="0" y2="40" stroke="#406280ff" stroke-width="2" stroke-dasharray="3"></line></svg>
			<div class="items-center">
				{#if bytesTotal === 0 || Math.round(progress * 100) === 100}
					<h2 class="text-xl font-bold">{status === 'submitted' ? "Odesláno" : "Nahráno"}</h2>
				{:else}
					<h2 class="text-xl">Nahráno {((bytesTotal / 1_000_000) * progress).toFixed(1)} MB</h2>
					<h2 class="text-xl self-center">z {(bytesTotal / 1_000_000).toFixed(1)} MB</h2>
				{/if}
			</div>
			<svg class="h-25" viewBox="0 0 2 40" xmlns="http://www.w3.org/2000/svg"><line x1="0" y="0" x2="0" y2="40" stroke="#406280ff" stroke-width="2" stroke-dasharray="3"></line></svg>
			<div class="items-center text-center">
				<h2 class="text-2xl text-sspsBlueDark font-bold mb-2">{Math.round(progress * 100)} %</h2>
				<ProgressBar progress={progress}></ProgressBar>
			</div>
		</div>
	{:else}
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
	{/if}
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
		@apply px-7 pb-7 pt-14;
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
