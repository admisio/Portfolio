<script lang="ts">
	import FullLayout from '$lib/components/layout/FullLayout.svelte';

	import woman from '$lib/assets/woman.png';
	import { onMount } from 'svelte';
	import axios from 'axios';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	
	
	let applicationId = Number($page.params.code);
	let codeValueMobile: string = '';
	let codeValueArray: Array<string> = [];
	let codeElementArray: Array<HTMLInputElement> = [];
			
	let loginFailed = false;


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

	const post = () => {
		axios({
			method: 'post',
			url: 'http://localhost:8000/candidate/login',
			data: {
				application_id: applicationId,
				password: codeValueMobile
			},
			withCredentials: true
		}).then((res) => {
			console.log(res);
			if (res.status === 200) {
				goto('/dashboard'); // TODO: Redirect to fill details first
			} else {
				loginFailed = true;
			}
		}).catch((err) => {
			loginFailed = true;
			// console.error(err);
		});
		console.log(codeValueMobile);
	}
	
	$: if (codeValueArray.length === 8) {
		post();
	};

	onMount(() => {
		codeElementArray[0].focus();
	});
</script>

<FullLayout>
	<div class="modal">
		<h2 class="mt-1 mx-8 fg-sspsBlueDark text-3xl text-center mb-8">
			Ev. č. přihlášky: <span class="font-bold"> {applicationId} </span>
		</h2>
		<img class="mx-auto" src={woman} alt="" />
		<div class="flex justify-center items-center">
			<input
				bind:value={codeValueMobile}
				class:codeInputDesktopLoginFailed={loginFailed}
				class:focus:border-sspsBlue={!loginFailed}
				type="text"
				class="codeInputMobile"
				
			/>
			{#each [1, 2, 3, 4] as value}
				<input
					class="codeInputDesktop"
					class:codeInputDesktopLoginFailed={loginFailed}
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
					class:codeInputDesktopLoginFailed={loginFailed}
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
		<input
			on:click={() => {
				post();
			}}
			class="w-full mt-8 md:hidden p-3 rounded-lg font-semibold text-xl transition-colors duration-300 bg-sspsBlue hover:bg-sspsBlueDark text-white hover:cursor-pointer"
			type="submit"
			value={'Přilásit'}
		/>
	</div>
</FullLayout>

<style>
	.modal {
		@apply flex flex-col items-center justify-center;
		@apply mx-auto my-auto;
		@apply w-[90vw] h-[85vh] md:w-4/5 md:h-4/5;
		@apply rounded-3xl;
		@apply bg-white;
	}
	input {
		@apply text-center font-semibold text-sspsBlue;
		@apply caret-transparent bg-[#f8fafb] shadow-lg p-3 rounded-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2;
		@apply <md:caret-current
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
	.codeInputDesktopLoginFailed {
		@apply border-red-700 border-4;
	}
</style>
