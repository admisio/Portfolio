<script lang="ts">
	import Telephone from '../icons/Telephone.svelte';
	import { tippy } from 'svelte-tippy';
	import 'tippy.js/dist/tippy.css';

	const helperText: string = 'Zadejte platný telefon s předvolbou. Například +420 123 456 789';
	export let placeholder: string = ''; // TODO

	import TelInput, { normalizedCountries } from 'svelte-tel-input';
	import type { NormalizedTelNumber, CountryCode, E164Number } from 'svelte-tel-input/types';

	// Any Country Code Alpha-2 (ISO 3166)
	let country: CountryCode | null = 'CZ';

	// You must use E164 number format. It's guarantee the parsing and storing consistency.
	export let value: E164Number | null = '+36301234567';

    // Validity
    let valid = true;
	export let invalid: boolean = false;
	$: invalid = !valid;

	// Optional - Extended details about the parsed phone number
	let parsedTelInput: NormalizedTelNumber | null = null;

	let selectedCountry: string | null = country;

	const countrySelect = (e: any) => {
		selectedCountry = e.target.value;
		// @ts-ignore
		country = selectedCountry;
		value = null;
	};

	const isTooltip = helperText ? tippy : () => {};
	$: tooltipDelay = invalid ? 0 : 1000;
</script>

<div class="wrapper w-full h-full flex"
	use:isTooltip={{
		content: helperText,
		placement: 'top',
		showOnCreate: false,
		delay: tooltipDelay
	}}
>
	<select
		class="countrySelect {!valid && 'invalid'}"
		aria-label="Default select example"
		name="Country"
		bind:value={selectedCountry}
		on:input={countrySelect}
	>
		<option value={null} hidden={selectedCountry !== null}>Země</option>
		{#each normalizedCountries as country (country.id)}
			<option
				value={country.iso2}
				selected={country.iso2 === selectedCountry}
				aria-selected={country.iso2 === selectedCountry}
			>
				{country.iso2} (+{country.dialCode})
			</option>
		{/each}
	</select>
	<div class="ml-2 inputWrapper">
		<TelInput bind:country bind:value bind:valid bind:parsedTelInput class="basic-tel-input {!valid ? 'invalid' : '' }" />
		<span>
			<Telephone />
		</span>
	</div>
</div>

<style lang="postcss">
	select {
		@apply h-full pl-3 pr-3 border-1 w-2/5 rounded;
		@apply hover:border-sspsBlue rounded-lg border border-2 bg-[#f8fafb] p-3 text-xl shadow-lg outline-none transition-colors  duration-300;
	}
	.inputWrapper {
		@apply w-full relative;
	}
	.inputWrapper span {
		@apply absolute right-0 top-1 bottom-0 my-auto flex bg-transparent p-3;
	}
  .wrapper :global(.basic-tel-input) {
      /* height: 32px;
      padding-left: 12px;
      padding-right: 12px;
      border-radius: 6px;
      border: 1px solid;
      outline: none;
	  width: 100%; */
	  /* @apply h-full pl-3 pr-3 border-1 w-full rounded; */
	  @apply hover:border-sspsBlue w-full rounded-lg border border-2 bg-[#f8fafb] p-3 text-xl shadow-lg outline-none transition-colors  duration-300;
  }

  .wrapper :global(.invalid) {
    /* border-color: red; */
	@apply border-red-700;
  }
</style>