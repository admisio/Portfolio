<script lang="ts">
	import LL from '$i18n/i18n-svelte';

	import FullLayout from '$lib/components/layout/FullLayout.svelte';

	import woman from '$lib/assets/woman.png';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { apiLogin } from '$lib/@api/candidate';
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';

	let applicationId = Number($page.params.code);
	let codeValueMobile: string = '';
	let codeValueArray: Array<string> = [];
	let codeElementArray: Array<HTMLInputElement> = [];
	let isError: boolean = false;

	$: {
		codeValueMobile = codeValueMobile.slice(0, 12);
		codeValueMobile = codeValueMobile.toUpperCase();
		codeValueArray = codeValueMobile.split('');
	}

	const inputDesktopOnKeyDown = async (index: number, e: KeyboardEvent) => {
		if (e.key === 'Backspace') {
			e.preventDefault();
			codeValueArray[index] = '';
			const prevElement = codeElementArray[index - 1];
			if (prevElement) codeElementArray[index - 1].focus();
		} else {
			if (e.key.length > 1 || e.metaKey || e.ctrlKey) return;
			e.preventDefault();
			codeValueArray[index] = e.key.toUpperCase();
			const nextElement = codeElementArray[index + 1];
			if (nextElement) codeElementArray[index + 1].focus();
		}
		codeValueMobile = codeValueArray.join('');
	};

	$: {
		if (codeValueMobile.length === 12) {
			submit();
		} else {
			isError = false;
		}
	}

	const submit = async () => {
		try {
			await apiLogin({ applicationId, password: codeValueMobile });
			goto('/dashboard');
		} catch (e) {
			console.error(e);
			toast.push('Neplatné heslo!', {
				theme: {
					'--toastColor': 'mintcream',
					'--toastBackground': '#b91c1c',
					'--toastBarBackground': '#7f1d1d'
				}
			});
			isError = true;
		}
	};

	const onPaste = async (e: ClipboardEvent) => {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain').slice(0, 12);
		if (text) codeValueMobile = text;
		for (const el of codeElementArray) {
			el.blur();
		}
	};

	onMount(() => {
		codeElementArray[0].focus();

		// Document on:paste
		document.addEventListener('paste', onPaste);

		return () => {
			// this function is called when the component is destroyed
			document.removeEventListener('paste', onPaste);
		};
	});
</script>

<FullLayout>
	<div class="modal">
		<img class="mx-auto" src={woman} alt="Woman Avatar" />
		<SvelteToast />
		<div class="flex items-center justify-center">
			<input
				class:error={isError}
				bind:value={codeValueMobile}
				type="text"
				class="codeInputMobile"
			/>
			{#each [0, 1, 2, 3] as value}
				<input
					class="codeInputDesktop"
					class:error={isError}
					bind:this={codeElementArray[value]}
					bind:value={codeValueArray[value]}
					on:keydown={(e) => inputDesktopOnKeyDown(value, e)}
					on:paste|preventDefault={(e) => onPaste(e)}
					type="text"
				/>
			{/each}
			<span class="separater" />
			{#each [4, 5, 6, 7] as value}
				<input
					class="codeInputDesktop"
					class:error={isError}
					bind:this={codeElementArray[value]}
					bind:value={codeValueArray[value]}
					on:keydown={(e) => inputDesktopOnKeyDown(value, e)}
					on:paste|preventDefault={(e) => onPaste(e)}
					type="text"
				/>
			{/each}
			<span class="separater" />
			{#each [8, 9, 10, 11] as value}
				<input
					class="codeInputDesktop"
					class:error={isError}
					bind:this={codeElementArray[value]}
					bind:value={codeValueArray[value]}
					on:keydown={(e) => inputDesktopOnKeyDown(value, e)}
					on:paste|preventDefault={(e) => onPaste(e)}
					type="text"
				/>
			{/each}
		</div>
		<h3 class="text-sspsBlue mx-8 mt-8 text-center text-xl font-semibold">
			{$LL.candidate.auth.application.title()}
		</h3>
		<p class="text-sspsGray mx-8 mt-8 text-center">
			{$LL.candidate.auth.application.help.description()}
			<u>{$LL.candidate.auth.application.help.here()}</u>
		</p>
	</div>
</FullLayout>

<style lang="postcss">
	.error {
		--at-apply: "border-red-700";
	}
	.modal {
		@apply flex flex-col items-center justify-center;
		@apply mx-auto my-auto;
		@apply h-full w-full;
		@apply bg-white;
		--at-apply: "md:rounded-3xl";
		--at-apply: "md:h-4/5 md:w-4/5";
	}
	input {
		@apply text-sspsBlue text-center font-semibold;
		@apply transition-colors duration-300;
		@apply caret-sspsBlueDark rounded-xl border border-2 bg-[#f8fafb] p-3 shadow-lg outline-none;
		--at-apply: "focus:border-sspsBlue hover:border-sspsBlue";
	}
	.separater {
		@apply bg-sspsBlue mr-2 hidden h-2 w-8;
		--at-apply: "md:block";
	}
	.codeInputMobile {
		@apply mx-5 w-full;
		--at-apply: "md:hidden";
	}
	.codeInputDesktop {
		@apply hidden;
		@apply mr-1;
		--at-apply: "md:mr-2";
		--at-apply: "sm:h-15 2xl:w-18 2xl:h-22 sm:w-12 sm:text-xl md:block md:h-20 md:w-16 md:text-4xl xl:h-20 xl:w-16 xl:p-0 xl:text-2xl";
	}
</style>
