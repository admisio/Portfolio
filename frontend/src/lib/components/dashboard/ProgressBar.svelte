<script lang="ts">
	export let progress: number;
	export let submitted: boolean = false;
</script>

<div class="progress-bar">
	<svg
		class="animated animate-ease-linear"
		width="40mm"
		height="8mm"
		viewBox="0 0 50 6"
		version="1.1"
		xmlns="http://www.w3.org/2000/svg"
	>
		<line x1="5" y1="3" x2="45" y2="3" stroke="#e6e6e6" stroke-width="6" stroke-linecap="round" />

		<line
			x1="5"
			y1="3"
			x2={progress * 45}
			y2="3"
			class:submitted={progress === 1 && submitted}
			class:not-submitted={progress === 1 && !submitted}
			class:uploading={progress !== 1}
			stroke-width="3"
			stroke-linecap="round"
		/>

		>
	</svg>
</div>

<style lang="postcss">
	.submitted {
		@apply stroke-[#35e000ff];
	}
	.not-submitted {
		@apply stroke-[#ef8b46];
	}
	.uploading {
		@apply stroke-[#75bff8ff];
	}
</style>
