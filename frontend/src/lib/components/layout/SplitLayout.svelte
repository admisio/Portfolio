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
		@apply min-w-screen fixed min-h-full md:min-w-[50vw];
		@apply bg-cover bg-no-repeat;
	}
	.bgOverlay {
		@apply min-w-screen fixed -z-10 min-h-full md:min-w-[50vw];
		background: linear-gradient(45deg, rgba(18, 48, 75, 1), rgba(119, 173, 224, 0.443));
		@apply bg-cover;
	}
	.bgOverlay .logo {
		@apply absolute left-0 top-0;
		@apply hidden md:inline-block;
		@apply max-w-56 h-auto;
		@apply p-7;
	}
	.darkModeToggle {
		@apply absolute right-0;
		@apply hidden md:inline-block;
		@apply p-7;
	}
	.view {
		@apply z-10;
		@apply absolute bottom-0 left-0 right-0 top-0 m-auto md:bottom-auto md:left-auto md:top-auto md:m-0;
		@apply md:h-screen md:w-[60vw];
		@apply md:my-auto;
		@apply bg-white;
	}
	.content {
		@apply h-full w-full;
	}
</style>
