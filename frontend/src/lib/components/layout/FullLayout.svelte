<script lang="ts">
	import backgroundImage from '$lib/assets/background.jpg';
	import logo from '$lib/assets/logo/ssps.svg';
	import DarkModeToggle from '../DarkModeToggle.svelte';
</script>

<div class="bg">
	<div class="bgOverlay">
		<img class="logo" src={logo}  alt="SSPŠ logo" />
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
		@apply w-[200px] h-[200px]
	}
	.bgImage {
		@apply -z-20;
		@apply absolute min-w-screen min-w-full min-h-screen;
		@apply bg-no-repeat;
		background-position: 55%;
	}
	.bgOverlay {
		@apply -z-10 absolute min-w-screen min-h-screen;
		background: linear-gradient(45deg, rgba(18, 48, 75, 1), rgba(119, 173, 224, 0.443));
		@apply bg-cover;
	}
	.bgOverlay .logo {
		@apply absolute top-0 left-0;
		@apply hidden md:inline-block;
		@apply h-auto max-w-72;
		@apply p-7;
	}
	.bgOverlay .darkModeToggle {
		@apply absolute top-0 right-0;
		@apply hidden md:inline-block;
		@apply p-7;
	}
	.view {
		@apply z-10 overflow-scroll;
		@apply w-screen h-screen top-0 right-0 bottom-0 left-0 m-auto;
	}
	.content {
		@apply w-full h-full;
		@apply flex items-center justify-center;
	}
</style>
