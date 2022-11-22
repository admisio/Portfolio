<script lang="ts">
	import FileType from './FileType.svelte';

	export let title: string;
	export let filetype: 'PDF' | 'ZIP';
	export let filesize: string;

    let dashAnimationProgress = 0;
    let dashAnimationInterval: NodeJS.Timer;

    const dashAnimationStart = () => {
        dashAnimationInterval = setInterval(() => {
            dashAnimationProgress += 1;
            if (dashAnimationProgress == 100) {
                dashAnimationProgress = 0;
            }
        }, 30);
    };

    const dashAnimationStop = () => {
        clearInterval(dashAnimationInterval);
    };

</script>

<div class="card uploadCard">
	<div class="flex flex-col sm:flex-row justify-between sm:items-center">
		<h3>{title}</h3>
		<div class="mt-3 sm:mt-0">
			<FileType {filetype} {filesize} />
		</div>
	</div>
	<div
		class="drag group"
        on:mouseenter={dashAnimationStart}
        on:mouseleave={dashAnimationStop}
		style={`background-image: url("data:image/svg+xml,%3csvg width='100%25' height='100%25' xmlns='http://www.w3.org/2000/svg'%3e%3crect width='100%25' height='100%25' fill='none' rx='9' ry='9'  stroke-opacity='50%' stroke='%23406280' stroke-width='4' stroke-dasharray='10' stroke-dashoffset='${dashAnimationProgress}' stroke-linecap='square'/%3e%3c/svg%3e");`}
	>
		<span class="text-[#406280]">Sem přetáhněte,</span>
		<span class="text-sspsGray">nebo nahrajte svůj motivační dopis</span>
	</div>
</div>

<style>
	.card {
		@apply h-full;
		@apply flex flex-col justify-between;

		@apply bg-[#f8fbfc];
		@apply rounded-3xl;
		@apply px-7 py-10;
	}
	.card h3 {
		@apply text-sspsBlue text-2xl xl:text-4xl font-semibold;
	}
	.card span {
		@apply opacity-60 text-sm;
        @apply transition-all duration-300
	}
	.card .drag {
        @apply transition duration-200;
		@apply flex-grow;
		@apply flex flex-col items-center justify-center;
		@apply mt-10;

		border-radius: 9px;
	}
    .card .drag:hover span{
        @apply opacity-100;
    }
</style>
