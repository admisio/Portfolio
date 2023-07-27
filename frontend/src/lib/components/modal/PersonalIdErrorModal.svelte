<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Modal from '../Modal.svelte';
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';

	let isOpened = true;
	export let personalIdNumber: string;

	const dispatch = createEventDispatcher();

	const close = () => {
		isOpened = false;
		dispatch('close');
	};
</script>

{#if isOpened}
	<SvelteToast />
	<Modal on:close={close}>
		<div class="relative h-[40vh] w-[80vw] flex flex-col items-center lg:w-[60vw]">
			<div>
				<h1 class="mt-6 text-center text-4xl font-bold text-sspsBlueDark">
					Vaše rodné číslo <span class="underline">není</span>
					'{personalIdNumber}'?
				</h1>
				<h3 class="mt-4 text-center">Znamená to, že jsme udělali chybu v přepisu přihlášky.</h3>
				<h2 class="mt-12 text-center text-2xl font-bold text-sspsBlueDark">
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
