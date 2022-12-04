<script lang="ts">
	import backgroundImage from '$lib/assets/background.jpg';
	import logo from '$lib/assets/logo/ssps.svg';
	import DarkModeToggle from '../DarkModeToggle.svelte';
</script>

<div class="bg">
	<div class="bgOverlay">
		<img class="logo" src={logo} alt="SSPŠ logo" />
		<div class="darkModeToggle">
			<DarkModeToggle backgroundColor="dark" />
		</div>
	</div>
	<div style={`background-image: url(${backgroundImage});`} class="bgImage" />
</div>
<div class="view">
	<div class="content">
		<slot />
	</div>
</div>

<style>
	.logo {
		@apply h-[200px] w-[200px];
	}
	.bgImage {
		@apply -z-20;
		@apply min-w-screen absolute min-h-screen min-w-full;
		@apply bg-no-repeat bg-cover;
		background-position: 55%;
	}
	.bgOverlay {
		@apply min-w-screen absolute -z-10 min-h-screen;
		background: linear-gradient(45deg, rgba(18, 48, 75, 1), rgba(119, 173, 224, 0.443));
		@apply bg-cover;
	}
	.bgOverlay .logo {
		@apply absolute top-0 left-0;
		@apply hidden md:inline-block;
		@apply max-w-72 h-auto;
		@apply p-7;
	}
	.bgOverlay .darkModeToggle {
		@apply absolute top-0 right-0;
		@apply hidden md:inline-block;
		@apply p-7;
	}
	.view {
		@apply z-10 overflow-scroll;
		@apply top-0 right-0 bottom-0 left-0 m-auto h-screen w-screen;
	}
	.content {
		@apply h-full w-full;
		@apply flex items-center justify-center;
	}
</style>
