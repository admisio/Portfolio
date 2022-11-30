<script lang="ts">
	import FullLayout from '$lib/components/layout/FullLayout.svelte';

	import woman from '$lib/assets/woman.png';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { login } from '../../../stores/candidate';
	import { goto } from '$app/navigation';
	
	
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
		codeValueMobile = codeValueArray.join('')
	};
	
	$: if (codeValueArray.length === 8) {
		submit();
	};

	async function submit() {
		try {
			await login({applicationId, password: codeValueMobile});
			goto("/register");
	 	} catch (e) {
			console.error(e);
		}
		// alert('ApplicationId: ' + applicationId + '; Password: ' + codeValueMobile);
	}

	onMount(() => {
		codeElementArray[0].focus();
	});
</script>

<FullLayout>
	<div class="modal">
		<img class="mx-auto" src={woman} alt="" />
		<div class="flex justify-center items-center">
			<input
				bind:value={codeValueMobile}
				type="text"
				class="codeInputMobile"
				
			/>
			{#each [1, 2, 3, 4] as value}
				<input
					class="codeInputDesktop"
					bind:this={codeElementArray[value - 1]}
					bind:value={codeValueArray[value - 1]}
					on:keydown|preventDefault={(e) => inputDesktopOnKeyDown(value - 1, e)}
					type="text"
				/>
			{/each}
			<span class="hidden sm:block mr-2 w-8 h-2 bg-sspsBlue" />
			{#each [5, 6, 7, 8] as value}
				<input
					class="codeInputDesktop"
					bind:this={codeElementArray[value - 1]}
					bind:value={codeValueArray[value - 1]}
					on:keydown|preventDefault={(e) => inputDesktopOnKeyDown(value - 1, e)}
					type="text"
				/>
			{/each}
		</div>
		<h3 class="mt-8 mx-8 text-sspsBlue font-semibold text-xl text-center">
			Zadejte 8místný kód pro aktivaci účtu
		</h3>
		<p class="mt-8 mx-8 text-sspsGray text-center">Nevíte si rady? Klikněte <u>zde</u></p>
	</div>
</FullLayout>

<style>
	.modal {
		@apply flex flex-col items-center justify-center;
		@apply mx-auto my-auto;
		@apply w-[90vw] h-[90vh] md:w-4/5 md:h-4/5;
		@apply rounded-3xl;
		@apply bg-white;
	}
	input {
		@apply text-center font-semibold text-sspsBlue;
		@apply caret-transparent bg-[#f8fafb] shadow-lg p-3 rounded-xl outline-none border transition-colors duration-300 focus:border-sspsBlue  hover:border-sspsBlue  border-2;
	}
	.codeInputMobile {
		@apply sm:hidden;
		@apply w-full mx-5;
	}
	.codeInputDesktop {
		@apply hidden;
		@apply mr-1 md:mr-2;
		@apply sm:block sm:text-xl sm:w-12 sm:h-15 md:text-4xl md:w-16 md:h-20 xl:text-4xl xl:w-18 xl:h-22 xl:p-0;
	}
</style>
