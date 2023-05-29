<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Modal from '../Modal.svelte';
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';

	export let applications: number[];
	let isOpened = true;

	const dispatch = createEventDispatcher();

	const close = () => {
		isOpened = false;
		dispatch('close');
	};
</script>

{#if isOpened}
	<SvelteToast />
	<Modal on:close={close}>
		<div class="relative flex h-[40vh] w-[80vw] flex-col items-center lg:w-[60vw]">
			<div>
				{#if applications.length === 1}
					<h1 class="text-sspsBlueDark mt-6 text-center text-4xl font-bold">
						Přihláška {applications[0]} <span class="underline">není</span>
						Vaše?
					</h1>
					<h1 class="text-sspsBlueDark mt-6 text-center text-4xl font-bold">
						Nebo jste podávali <span class="underline">dvě</span>
						přihlášky?
					</h1>
				{:else}
					<h1 class="text-sspsBlueDark mt-6 text-center text-4xl font-bold">
						Přihlášky {applications[0]} a {applications[1]} <span class="underline">nejsou</span>
						Vaše?
					</h1>
					<h1 class="text-sspsBlueDark mt-6 text-center text-4xl font-bold">
						Nebo jste podávali jen <span class="underline">jednu</span>
						přihlášku?
					</h1>
				{/if}
				<h3 class="mt-4 text-center">Znamená to, že jsme udělali chybu v přepisu přihlášky.</h3>
				<h2 class="text-sspsBlueDark mt-12 text-center text-2xl font-bold">
					Kontaktujte nás na telefonu +420 123 456 789,
				</h2>
				<h3 class="mt-2 text-center">
					nebo nám napište na email na
					<a href="mailto:portfolio@ssps.cz " class="underline"> portfolio@ssps.cz </a>
				</h3>
				<h4 class="absolute bottom-4 left-0 w-full text-center">
					Omlouváme se za komplikace s přijímacím řízením SSPŠaG
				</h4>
			</div>
		</div>
	</Modal>
{/if}

<style lang="postcss">
</style>
