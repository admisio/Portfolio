<script lang="ts">
	import backgroundImage from '$lib/assets/background.jpg';
	import logo from '$lib/assets/logo/ssps.svg';
	import DarkModeToggle from '../DarkModeToggle.svelte';
</script>

<div class="bg">
	<div class="bgOverlay">
		<img class="logo" src={logo} alt="SSPŠ logo" />
	</div>
	<div style={`background-image: url(${backgroundImage});`} class="bgImage" />
</div>
<div class="view">
    <div class="darkModeToggle">
        <DarkModeToggle/>
    </div>
	<div class="content">
		<slot />
	</div>
</div>

<style>
	.bgImage {
		@apply -z-20;
		@apply absolute min-w-screen md:min-w-[50vw] min-h-screen;
		@apply bg-no-repeat;
		background-position: 55%;
	}
	.bgOverlay {
		@apply -z-10 absolute min-w-screen md:min-w-[50vw] min-h-screen;
		background: linear-gradient(45deg, rgba(18, 48, 75, 1), rgba(119, 173, 224, 0.443));
		@apply bg-cover;
	}
	.bgOverlay .logo {
		@apply absolute top-0 left-0;
		@apply hidden md:inline-block;
		@apply h-auto max-w-56;
		@apply p-7;
	}
	.darkModeToggle {
		@apply absolute right-0;
		@apply hidden md:inline-block;
		@apply p-7;
	}
	.view {
		@apply z-10 overflow-scroll;
		@apply rounded-3xl md:rounded-none;
		@apply absolute w-[90vw] h-[85vh] top-0 md:top-auto right-0 bottom-0 md:bottom-auto left-0 md:left-auto m-auto md:m-0;
		@apply md:w-[50vw] md:h-screen;
		@apply md:my-auto;
		@apply bg-white;
	}
    .content {
        @apply w-full h-full;
    }
</style>
