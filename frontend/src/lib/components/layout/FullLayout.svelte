<script lang="ts">
	import backgroundImage from '$lib/assets/background.jpg';
	import logo from '$lib/assets/logo/ssps.svg';
	import DarkModeToggle from '../DarkModeToggle.svelte';

	export let hideHeader: boolean = false;
</script>

<div class="bg">
	<div class="bgOverlay">
		{#if !hideHeader}
			<img class="logo" src={logo} alt="SSPŠ logo" />
			<div class="darkModeToggle">
				<DarkModeToggle backgroundColor="dark" />
			</div>
		{/if}
	</div>
	<div style={`background-image: url(${backgroundImage});`} class="bgImage" />
</div>
<div class="view">
	<div class="content">
		<slot />
	</div>
</div>

<style lang="postcss">
	.logo {
		@apply h-[200px] w-[200px];
	}
	.bgImage {
		@apply -z-20;
		@apply min-w-screen fixed min-h-screen min-w-full;
		@apply bg-cover bg-no-repeat;
		background-position: 55%;
	}
	.bgOverlay {
		@apply min-w-screen fixed -z-10 min-h-screen;
		background: linear-gradient(45deg, rgba(18, 48, 75, 1), rgba(119, 173, 224, 0.443));
		@apply bg-cover;
	}
	.bgOverlay .logo {
		@apply absolute left-0 top-0;
		@apply hidden;
		@apply max-w-72 h-auto;
		@apply p-7;
		--at-apply: 'md:inline-block';
	}
	.bgOverlay .darkModeToggle {
		@apply absolute right-0 top-0;
		@apply hidden;
		@apply p-7;
		--at-apply: 'md:inline-block';
	}
	.view {
		@apply z-10;
		@apply bottom-0 left-0 right-0 top-0 m-auto h-screen w-screen;
	}
	.content {
		@apply h-full w-full;
		@apply flex items-center justify-center;
	}
</style>
