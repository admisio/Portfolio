<script lang="ts">
	import FullLayout from '$lib/components/layout/FullLayout.svelte';

	import woman from '$lib/assets/woman.png';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { apiLogin } from '$lib/@api/candidate';

	let applicationId = Number($page.params.code);
	let codeValueMobile: string = '';
	let codeValueArray: Array<string> = [];
	let codeElementArray: Array<HTMLInputElement> = [];

	$: {
		codeValueMobile = codeValueMobile.toUpperCase();
		codeValueArray = codeValueMobile.split('');
		console.log(codeValueArray);
	}

	const inputDesktopOnKeyDown = (index: number, e: KeyboardEvent) => {
		if (e.key === 'Backspace') {
			codeValueArray[index] = '';
			if (codeElementArray[index - 1]) {
				codeElementArray[index - 1].focus();
			}
		} else {
			if (e.key.length > 1) {
				return;
			}
			codeValueArray[index] = e.key.toUpperCase();
			if (codeElementArray[index + 1]) {
				codeElementArray[index + 1].focus();
			}
		}
		codeValueMobile = codeValueArray.join('');
	};

	$: if (codeValueArray.length === 12) {
		submit();
	}

	const submit = async () => {
		try {
			await apiLogin({ applicationId, password: codeValueMobile });
			goto('/dashboard');
		} catch (e) {
			console.error(e);
		}
		// alert('ApplicationId: ' + applicationId + '; Password: ' + codeValueMobile);
	};

	const onPaste = (e: ClipboardEvent) => {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain');
		if (text) {
			codeValueMobile = text;
		}
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
		<img class="mx-auto" src={woman} alt="" />
		<div class="flex items-center justify-center">
			<input bind:value={codeValueMobile} type="text" class="codeInputMobile" />
			{#each [1, 2, 3, 4] as value}
				<input
					class="codeInputDesktop"
					bind:this={codeElementArray[value - 1]}
					bind:value={codeValueArray[value - 1]}
					on:keydown={(e) => inputDesktopOnKeyDown(value - 1, e)}
					on:paste|preventDefault={(e) => onPaste(e)}
					type="text"
				/>
			{/each}
			<span class="separater" />
			{#each [5, 6, 7, 8] as value}
				<input
					class="codeInputDesktop"
					bind:this={codeElementArray[value - 1]}
					bind:value={codeValueArray[value - 1]}
					on:keydown={(e) => inputDesktopOnKeyDown(value - 1, e)}
					on:paste|preventDefault={(e) => onPaste(e)}
					type="text"
				/>
			{/each}
			<span class="separater" />
			{#each [9, 10, 11, 12] as value}
				<input
					class="codeInputDesktop"
					bind:this={codeElementArray[value - 1]}
					bind:value={codeValueArray[value - 1]}
					on:keydown={(e) => inputDesktopOnKeyDown(value - 1, e)}
					on:paste|preventDefault={(e) => onPaste(e)}
					type="text"
				/>
			{/each}
		</div>
		<h3 class="text-sspsBlue mx-8 mt-8 text-center text-xl font-semibold">
			Zadejte 12místný kód pro aktivaci účtu
		</h3>
		<p class="text-sspsGray mx-8 mt-8 text-center">Nevíte si rady? Klikněte <u>zde</u></p>
	</div>
</FullLayout>

<style lang="postcss">
	.modal {
		@apply flex flex-col items-center justify-center;
		@apply mx-auto my-auto;
		@apply h-full w-full md:h-4/5 md:w-4/5;
		@apply md:rounded-3xl;
		@apply bg-white;
	}
	input {
		@apply text-sspsBlue text-center font-semibold;
		@apply transition-colors duration-300;
		@apply focus:border-sspsBlue hover:border-sspsBlue rounded-xl border border-2 bg-[#f8fafb] p-3 md:caret-transparent shadow-lg outline-none;
	}
	.separater {
		@apply bg-sspsBlue mr-2 hidden h-2 w-8 md:block;
	}
	.codeInputMobile {
		@apply md:hidden;
		@apply mx-5 w-full;
	}
	.codeInputDesktop {
		@apply hidden;
		@apply mr-1 md:mr-2;
		@apply sm:h-15 2xl:w-18 2xl:h-22 sm:w-12 sm:text-xl md:block md:h-20 md:w-16 md:text-4xl xl:h-20 xl:w-16 xl:p-0;
	}
</style>
