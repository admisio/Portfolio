<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Input from './input';

	const dispatch = createEventDispatcher();

	export let type: 'text' | 'number' | 'tel' | 'e-mail' = 'text';
	export let validatorType: 'default' | 'email' | 'tel' | 'name' | 'number' | 'birthdate' | 'personalIdNumber' = 'default';
	const typeAction = (node: HTMLInputElement) => {
		node.type = type;
	};
	export let placeholder: string = '';
	export let value: string = '';

	export let input = new Input();
	if (input.value === '') {
		input.value = value;
	}


	$: {
		if (validatorType === 'tel') {
			let x = input.value.replace(/\D/g, '').match(/(\d{0,3})(\d{0,3})(\d{0,3})(\d{0,3})/)!;
			input.value = '+' + x[1]  + (x[2] ? ' ' + x[2] : '') + (x[3] ? ' ' + x[3] : '') + (x[4] ? ' ' + x[4] : '');
		} else if (validatorType === 'number') {
			input.value = input.value.replace(/[^0-9]/g, '');
		} else if (validatorType === 'birthdate') { // TODO: more intuitive date input
			let x = input.value.replace(/\D/g, '').match(/(\d{0,2})(\d{0,2})(\d{0,4})/)!;
			input.value = x[1] + (x[2] ? '.' + x[2] : '') + (x[3] ? '.' + x[3] : '');
		} else if (validatorType === 'personalIdNumber') {
			let x = input.value.replace(/\D/g, '').match(/(\d{0,6})(\d{0,4})/)!;
			input.value = x[1] + (x[2] ? '/' + x[2] : '');
		}
	}

	const onInput = () => {
		// input.value = value;
		input.isValid = validate();
	}

	const validate = (): boolean => {
		if (validatorType === 'email') {
			return (/^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/)
				.test(input.value);
		} else if (validatorType === 'tel') {
			return (/^\+?\d{1,3} ?\d{3} ?\d{3} ?\d{3}$/)
				.test(input.value);
		} else if (validatorType === 'name') {
			return (/^[a-zA-ZàáâäãåąčćęèéêëėįìíîïłńòóôöõøùúûüųūÿýżźñçčšžÀÁÂÄÃÅĄĆČĖĘÈÉÊËÌÍÎÏĮŁŃÒÓÔÖÕØÙÚÛÜŲŪŸÝŻŹÑßÇŒÆČŠŽ∂ð ,.'-]+$/u)
				.test(input.value);
		} else if (validatorType === 'number') {
			return (/^[0-9]+$/)
				.test(input.value);
		} else {
			return true;
		}
	};

	export let icon: boolean = false;
</script>

<div class="mt-8 <md:mt-4 relative flex justify-center items-center">
	<input
		bind:value={input.value}
		on:click
		on:keydown
		on:keyup
		on:input={onInput}
		class:withIcon={icon}
		class:invalid={!input.isValid}
		class="bg-[#f8fafb] w-full shadow-lg p-3 rounded-lg text-xl outline-none border transition-colors duration-300 hover:border-sspsBlue  border-2"
		use:typeAction
		{placeholder}
	/>
	{#if icon}
		<span class="flex absolute right-0 top-0 bottom-0 my-auto bg-transparent p-3">
			<slot name="icon" />
		</span>
	{/if}
</div>

<style>
	.withIcon {
		@apply pr-14;
	}
	.invalid {
		@apply border-2 border-red-500;
	}
</style>
