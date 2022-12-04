<script lang="ts">
	import FileType from './FileType.svelte';
	import FileDrop from 'filedrop-svelte';
	import { submissionProgress, UploadStatus, type Status } from '$lib/stores/portfolio';
	import { createEventDispatcher } from 'svelte';
	import ProgressBar from './ProgressBar.svelte';
	import type { AxiosProgressEvent } from 'axios';
	import StatusNotificationDot from './StatusNotificationDot.svelte';

	import documentIcon from '$lib/assets/document.png';
	import archiveIcon from '$lib/assets/archive.png';

	const dispatch = createEventDispatcher();

	export let title: string;
	export let filetype: 'PDF' | 'ZIP';
	export let filesize: number;
	export let fileType: number;
	export let placeholder: string = '';

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
				if ($submissionProgress.files!.some((code) => code === fileType)) {
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
	};

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

<div class="card uploadCard relative">
	<div class="header">
		<h3 class="mb-4 sm:mb-0">{title}</h3>
		<div class="mb-4 mt-1 sm:mb-0 sm:mt-0">
			<FileType {filetype} filesize={filesize + ' MB'} />
		</div>
		<div class="absolute right-0 top-4 px-7">
			<StatusNotificationDot {status} />
		</div>
	</div>
	{#if fileDropped}
		<div class="body uploaded flex content-around items-center justify-between">
			<div class="w-24">
				<img
					class="w-full object-scale-down"
					src={filetype == 'PDF' ? documentIcon : archiveIcon}
					alt="Icon"
				/>
			</div>
			<svg class="h-25 hidden xl:block" viewBox="0 0 2 40" xmlns="http://www.w3.org/2000/svg"
				><line
					x1="0"
					y="0"
					x2="0"
					y2="40"
					stroke="#406280ff"
					stroke-width="2"
					stroke-dasharray="3"
				/></svg
			>
			<div class="hidden items-center xl:block">
				{#if bytesTotal === 0 || Math.round(progress * 100) === 100}
					<h2 class="text-xl font-bold">{status === 'submitted' ? 'Odesláno' : 'Nahráno'}</h2>
				{:else}
					<h2 class="text-xl">Nahráno {((bytesTotal / 1_000_000) * progress).toFixed(1)} MB</h2>
					<h2 class="self-center text-xl">z {(bytesTotal / 1_000_000).toFixed(1)} MB</h2>
				{/if}
			</div>
			<svg class="h-25" viewBox="0 0 2 40" xmlns="http://www.w3.org/2000/svg"
				><line
					x1="0"
					y="0"
					x2="0"
					y2="40"
					stroke="#406280ff"
					stroke-width="2"
					stroke-dasharray="3"
				/></svg
			>
			<div class="items-center text-center">
				<h2 class="text-sspsBlueDark mb-2 text-2xl font-bold">{Math.round(progress * 100)} %</h2>
				<ProgressBar {progress} />
			</div>
		</div>
	{:else}
		<div class="body">
			<FileDrop
				multiple={false}
				maxSize={filesize * 1_000_000}
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
					<span class="text-sspsGray">nebo nahrajte {placeholder}</span>
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
		@apply m-3 bg-transparent;
		@apply h-full;
		@apply flex flex-col justify-between;

		@apply rounded-3xl;

		@apply transition-all duration-300;
	}
	.card:hover {
		@apply shadow-2xl;
		@apply m-0;
	}
	.header {
		@apply rounded-t-3xl;
		@apply px-7 pb-7 pt-14;
		background-color: rgb(220, 238, 253);
		backdrop-filter: blur(15px) saturate(0.86);
		-webkit-backdrop-filter: blur(15px) saturate(0.86);

		@apply flex flex-col justify-between sm:flex-row sm:items-center;
	}
	.body {
		@apply bg-[#f8fbfc];
		@apply rounded-b-3xl;
		@apply flex-1;
		@apply p-7;
	}
	.uploaded {
		@apply 2xl:px-14 ;
	}
	.card h3 {
		@apply text-sspsBlue text-2xl font-semibold xl:text-4xl;
	}
	.card span {
		@apply text-sm opacity-60;
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
