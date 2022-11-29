<script lang="ts">
	import { goto } from '$app/navigation';

	import lev from '$lib/assets/logo/lev.png';
	import Email from '$lib/components/icons/Email.svelte';
	import Home from '$lib/components/icons/Home.svelte';
	import Telephone from '$lib/components/icons/Telephone.svelte';
	import SplitLayout from '$lib/components/layout/SplitLayout.svelte';
	import IdField from '$lib/components/textfield/IdField.svelte';
	import TelephoneField from '$lib/components/textfield/TelephoneField.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';

	import { createForm } from 'svelte-forms-lib';
	import * as yup from 'yup';

	let applicationValue = '';

	const pageCount = 3;
	let pageIndex = 2;
	let pagesFilled = 0;

	const formInitialValues = {
		name: '',
		email: '',
		telephone: '',
		birthSurname: '',
		birthPlace: '',
		birthDate: '',
		sex: '',
		address: '',
		parentEmail: '',
		parentTelephone: '',
		citizenship: '',
		personalId: '',
		study: '',
		applicationId: ''
	};

	const { form, errors, state, handleChange, handleSubmit } = createForm({
		initialValues: formInitialValues,
		validationSchema: yup.object().shape({
			name: yup.string().required(),
			email: yup.string().email().required(),
			telephone: yup
				.string()
				.required()
				.matches(/^\+\d{1,3} \d{3} \d{3} \d{3}$/),
			birthSurname: yup.string().required(),
			birthPlace: yup.string().required(),
			birthDate: yup.string().required(),
			sex: yup.string().required(),
			address: yup.string().required(),
			parentEmail: yup.string().email().required(),
			parentTelephone: yup.string().required(),
			citizenship: yup.string().required(),
			personalId: yup.string().required(),
			study: yup.string().required(),
			applicationId: yup.string().required()
		}),
		onSubmit: (values) => {
			alert(JSON.stringify(values));
		}
	});

	const isPageInvalid = (): boolean => {
		switch (pageIndex) {
			case 0:
				if ($errors.name || $errors.email || $errors.telephone) {
					return true;
				}
				break;

			case 1:
				if ($errors.birthSurname || $errors.birthPlace || $errors.birthDate || $errors.sex) {
					return true;
				}
				break;
			case 2:
				if ($errors.address || $errors.parentEmail || $errors.parentTelephone) {
					return true;
				}
				break;
			case 3:
				if ($errors.citizenship || $errors.personalId || $errors.study || $errors.applicationId) {
					return true;
				}
				break;
			default:
				return false;
		}
		return false;
	};
</script>

<SplitLayout>
	<div class="form">
		<div
			class="py-3 px-6 md:py-4 md:px-8 rounded-[999px] shadow-2xl flex items-center justify-center"
		>
			<img class="object-cover" src={lev} alt="" />
		</div>
		{#if pageIndex === 0}
			<form on:submit={handleSubmit}>
				<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Registrace</h1>
				<p class="block mt-8 font-light text-sspsGray text-center">
					Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget
					elit.
				</p>
				<div class="flex md:flex-col items-center justify-center w-full">
					<span class="w-full mt-8">
						<TextField
							error={$errors.name}
							on:change={handleChange}
							bind:value={$form.name}
							type="text"
							placeholder="Jméno a příjmení"
						/>
					</span>
					<span class="w-full mt-8 ml-2 md:ml-0">
						<TextField
							error={$errors.email}
							on:change={handleChange}
							bind:value={$form.email}
							type="e-mail"
							placeholder="Email"
							icon
						>
							<div slot="icon" class="flex items-center justify-center">
								<Email />
							</div>
						</TextField>
					</span>
				</div>
				<div class="mt-8 w-full">
					<TelephoneField
						error={$errors.telephone}
						on:change={handleChange}
						bind:value={$form.telephone}
						placeholder="Telefon"
					/>
				</div>
			</form>
		{/if}
		{#if pageIndex === 1}
			<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Něco o tobě</h1>
			<p class="block mt-8 font-light text-sspsGray text-center">
				Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
			</p>
			<div class="flex flex-col w-full">
				<span class="w-full mt-8">
					<TextField
						type="text"
						placeholder="Rodné příjmení"
						error={$errors.birthSurname}
						on:change={handleChange}
						bind:value={$form.birthSurname}
					/>
				</span>
				<span class="w-full mt-8">
					<TextField
						error={$errors.birthPlace}
						on:change={handleChange}
						bind:value={$form.birthPlace}
						type="text"
						placeholder="Místo narození"
						icon
					>
						<div slot="icon" class="flex items-center justify-center">
							<Home />
						</div>
					</TextField>
				</span>
			</div>

			<div class="mt-8 flex items-center w-full">
				<TextField
					error={$errors.birthDate}
					on:change={handleChange}
					bind:value={$form.birthDate}
					type="text"
					placeholder="Datum narození"
				/>
				<div class="ml-2">
					<TextField
						error={$errors.sex}
						on:change={handleChange}
						bind:value={$form.sex}
						type="text"
						placeholder="Pohlaví"
					/>
				</div>
			</div>
		{/if}
		{#if pageIndex === 2}
			<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Už jen kousek!</h1>
			<p class="block mt-8 font-light text-sspsGray text-center">
				Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
			</p>
			<div class="flex flex-col w-full">
				<span class="w-full mt-8">
					<TextField
						error={$errors.address}
						on:change={handleChange}
						bind:value={$form.address}
						type="text"
						placeholder="Adresa trvalého bydliště"
					/>
				</span>
				<div class="mt-8 flex flex-row items-center md:flex-col">
					<span class="w-full">
						<TextField
							error={$errors.parentEmail}
							on:change={handleChange}
							bind:value={$form.parentEmail}
							type="e-mail"
							placeholder="E-mail zákonného zástupce"
						/>
					</span>
					<span class="w-full ml-2 md:ml-0 md:mt-8">
						<TelephoneField
							error={$errors.parentTelephone}
							on:change={handleChange}
							bind:value={$form.parentTelephone}
							placeholder="Telefon zákonného zástupce"
						/>
					</span>
				</div>
			</div>
		{/if}
		{#if pageIndex === 3}
			<h1 class="mt-8 text-4xl text-sspsBlue font-semibold">Poslední krok</h1>
			<p class="block mt-8 font-light text-sspsGray text-center">
				Lorem ipsum dolor sit amet, consectetuer adipiscing elit.<br /> Fusce suscipit libero eget elit.
			</p>
			<div class="flex flex-col w-full">
				<span class="w-full mt-8">
					<TextField
						error={$errors.citizenship}
						on:change={handleChange}
						bind:value={$form.citizenship}
						type="text"
						placeholder="Občanství"
					/>
				</span>
			</div>
			<div class="mt-8 flex items-center justify-center w-full">
				<IdField
					error={$errors.personalId}
					on:change={handleChange}
					bind:value={$form.personalId}
					placeholder="Rodné číslo"
				/>
				<TextField
					error={$errors.study}
					on:change={handleChange}
					bind:value={$form.study}
					type="text"
					placeholder="Obor"
				/>
			</div>
			<div class="mt-8 flex flex-col w-full">
				<TextField
					error={$errors.applicationId}
					on:change={handleChange}
					bind:value={$form.applicationId}
					type="text"
					placeholder="Evidenční číslo přihlášky"
				/>
			</div>
		{/if}
		<input
			on:click={async (e) => {
				await handleSubmit(e);
				if (isPageInvalid()) return;
				if (pageIndex === pageCount) {
					alert('should submit');
				} else {
					pagesFilled++;
					pageIndex++;
				}
				errors.set(formInitialValues);
			}}
			class="w-full mt-8 p-3 rounded-lg font-semibold text-xl transition-colors duration-300 bg-sspsBlue hover:bg-sspsBlueDark text-white hover:cursor-pointer"
			type="submit"
			value={pageIndex === pageCount ? 'Odeslat' : 'Pokračovat'}
		/>

		<div class="mt-8 flex flex-row justify-center">
			{#each Array(pageCount + 1) as _, i}
				<button
					class:dotActive={i === pageIndex}
					on:click={async (e) => {
						if (i <= pagesFilled) {
							// never skip unfilled or invalid pages
							pageIndex = i;
						} else if (i == pagesFilled + 1) {
							// if next page is clicked, validate current page
							await handleSubmit(e);
							if (isPageInvalid()) return;
							pagesFilled++;
							pageIndex++;
							errors.set(formInitialValues);
						}
					}}
					class="dot"
				/>
			{/each}
		</div>
	</div>
</SplitLayout>

<style>
	.form {
		@apply flex flex-col;
		@apply mx-auto w-[90%] h-full;
		@apply items-center justify-center;
	}
	.form > form {
		@apply flex flex-col;
		@apply w-full;
		@apply items-center justify-center;
	}
	.dot {
		@apply ml-2 w-4 h-4 
		@apply hover:cursor-pointer hover:bg-sspsBlue
		@apply rounded-full bg-sspsGray;
	}
	.dotActive {
		@apply bg-sspsBlue;
	}
</style>
