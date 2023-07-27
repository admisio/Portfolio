<script lang="ts">
	import defaultBg from '$lib/assets/background.jpg';
	import logo from '$lib/assets/logo/ssps.svg';
	import DarkModeToggle from '../DarkModeToggle.svelte';

	export let backgroundImage: string = defaultBg;
	export let backgroundPosition: string = '55%';
</script>

<div class="bg">
	<div class="bgOverlay">
		<img class="logo" src={logo} alt="SSPŠ logo" />
	</div>
	<div
		style={`background-image: url(${backgroundImage}); background-position: ${backgroundPosition}`}
		class="bgImage"
	/>
</div>
<div class="view">
	<div class="darkModeToggle">
		<DarkModeToggle />
	</div>
	<div class="content">
		<slot />
	</div>
</div>

<style lang="postcss">
	.bgImage {
		@apply -z-20;
		@apply min-w-screen fixed min-h-full;
		@apply bg-cover bg-no-repeat;
		--at-apply: "md:min-w-[50vw]";
	}
	.bgOverlay {
		@apply min-w-screen fixed -z-10 min-h-full ;
		@apply bg-cover;
		--at-apply:"md:min-w-[50vw]";
		background: linear-gradient(45deg, rgba(18, 48, 75, 1), rgba(119, 173, 224, 0.443));
	}
	.bgOverlay .logo {
		@apply absolute left-0 top-0;
		@apply hidden;
		@apply max-w-56 h-auto;
		@apply p-7;
		--at-apply: "md:inline-block";
	}
	.darkModeToggle {
		@apply absolute right-0;
		@apply hidden;
		@apply p-7;
		--at-apply: "md:inline-block";
	}
	.view {
		@apply z-10;
		@apply absolute bottom-0 left-0 right-0 top-0 m-auto;
		--at-apply: "md:bottom-auto md:left-auto md:top-auto md:m-0 md:my-auto";
		--at-apply: "md:h-screen md:w-[60vw]";
		--at-apply: "md:bg-white";
	}
	.content {
		@apply h-full w-full;
	}
</style>
