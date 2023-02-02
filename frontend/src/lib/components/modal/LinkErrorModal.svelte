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
		<div class="w-[80vw] lg:w-[60vw] h-[40vh] flex flex-col items-center relative">
            <div>
				{#if applications.length === 1}
					<h1 class="mt-6 text-4xl text-center text-sspsBlueDark font-bold">
						Přihláška {applications[0]} <span class="underline">není</span>
						Vaše?
					</h1>
					<h1 class="mt-6 text-4xl text-center text-sspsBlueDark font-bold">
						Nebo jste podávali <span class="underline">dvě</span>
						přihlášky?
					</h1>
				{:else}
					<h1 class="mt-6 text-4xl text-center text-sspsBlueDark font-bold">
						Přihlášky {applications[0]} a {applications[1]} <span class="underline">nejsou</span>
						Vaše?
					</h1>
					<h1 class="mt-6 text-4xl text-center text-sspsBlueDark font-bold">
						Nebo jste podávali jen <span class="underline">jednu</span>
						přihlášku?
					</h1>
				{/if}
                <h3 class="text-center mt-4"
                    >Znamená to, že jsme udělali chybu v přepisu přihlášky.
                </h3>
                <h2 class="font-bold text-2xl text-center text-sspsBlueDark mt-12">
                    Kontaktujte nás na telefonu +420 123 456 789,
                </h2>
                <h3 class="text-center mt-2">
                    nebo nám napište na email na
                    <a href="mailto:portfolio@ssps.cz " class="underline"> portfolio@ssps.cz </a>
                </h3>
                <h4 class="text-center absolute bottom-4 left-0 w-full">Omlouváme se za komplikace s přijímacím řízením SSPŠaG</h4>
            </div>
        </div>
	</Modal>
{/if}

<style lang="postcss">
</style>
