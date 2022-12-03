<script lang="ts">
	import { goto } from '$app/navigation';
	import { apiFillDetails } from '$lib/@api/candidate';

	import Email from '$lib/components/icons/Email.svelte';
	import Home from '$lib/components/icons/Home.svelte';
	import SchoolBadge from '$lib/components/icons/SchoolBadge.svelte';
	import Telephone from '$lib/components/icons/Telephone.svelte';
	import SplitLayout from '$lib/components/layout/SplitLayout.svelte';
	import EmailField from '$lib/components/textfield/EmailField.svelte';
	import IdField from '$lib/components/textfield/IdField.svelte';
	import TelephoneField from '$lib/components/textfield/TelephoneField.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';

	import { createForm } from 'svelte-forms-lib';
	import * as yup from 'yup';

	const pageCount = 3;
	let pageIndex = 0;
	let pagesFilled = 0;

	const formInitialValues = {
		name: '',
		surname: '',
		email: '',
		telephone: '',
		birthplace: '',
		birthdate: '',
		sex: '',
		address: '',
		citizenship: '',
		personalIdNumber: '',
		study: '',
		parentName: 'TODO name',
		parentSurname: 'TODO',
		parentTelephone: '',
		parentEmail: ''
	};

	const { form, errors, handleSubmit, handleChange } = createForm({
		initialValues: formInitialValues,
		validationSchema: yup.object().shape({
			name: yup.string().required(),
			surname: yup.string(),
			email: yup.string().email().required(),
			telephone: yup
				.string()
				.required()
				.matches(/^\+\d{1,3} \d{3} \d{3} \d{3}$/),
			birthplace: yup.string().required(),
			birthdate: yup.string().required(),
			sex: yup.string(),
			address: yup.string().required(),
			citizenship: yup.string().required(),
			personalIdNumber: yup.string().required(),
			study: yup.string().required(),
			parentName: yup.string(),
			parentSurname: yup.string(),
			parentTelephone: yup
				.string()
				.required()
				.matches(/^\+\d{1,3} \d{3} \d{3} \d{3}$/),
			parentEmail: yup.string().email().required()
		}),

		onSubmit: async (values) => {
			if (pageIndex === pageCount) {
				try {
					console.log('submit');
					// @ts-ignore // love javascript
					delete values.undefined;
					values.birthdate = '2000-01-01'; // TODO: reformat user typed date
					await apiFillDetails(values);
					goto('/dashboard');
				} catch (e) {
					console.error('error while submitting data: ' + e);
				}
			}
		}
	});

	$: console.log($errors);

	const isPageInvalid = (): boolean => {
		switch (pageIndex) {
			case 0:
				if ($errors.name || $errors.email || $errors.telephone) {
					return true;
				}
				break;

			case 1:
				if (
					/* $errors.birthdurname || */ $errors.birthplace ||
					$errors.birthdate /* || $errors.sex */
				) {
					return true;
				}
				break;
			case 2:
				if ($errors.address || $errors.parentEmail || $errors.parentTelephone) {
					return true;
				}
				break;
			case 3:
				if (
					$errors.citizenship ||
					$errors.personalIdNumber ||
					$errors.study //||
					// $errors.applicationId
				) {
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
		<div class="w-24 h-24 md:w-auto md:h-auto">
			<SchoolBadge />
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
						<EmailField
							error={$errors.email}
							on:change={handleChange}
							bind:value={$form.email}
							placeholder="E-mail"
						/>
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
			<div class="flex flex-row md:flex-col w-full">
				<span class="w-full mt-8">
					<TextField type="text" placeholder="Rodné příjmení" on:change={handleChange} />
				</span>
				<span class="w-full mt-8 ml-2 md:ml-0">
					<TextField
						error={$errors.birthplace}
						on:change={handleChange}
						bind:value={$form.birthplace}
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
					error={$errors.birthdate}
					on:change={handleChange}
					bind:value={$form.birthdate}
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
						<EmailField
							error={$errors.parentEmail}
							on:change={handleChange}
							bind:value={$form.parentEmail}
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
			<div class="flex flex-row md:flex-col w-full">
				<span class="w-full mt-8">
					<TextField
						error={$errors.citizenship}
						on:change={handleChange}
						bind:value={$form.citizenship}
						type="text"
						placeholder="Občanství"
					/>
				</span>
				<span class="w-full mt-8 ml-2 md:ml-0">
					<TextField on:change={handleChange} type="text" placeholder="Evidenční číslo přihlášky" />
				</span>
			</div>
			<div class="mt-8 flex items-center justify-center w-full">
				<IdField
					error={$errors.personalIdNumber}
					on:change={handleChange}
					bind:value={$form.personalIdNumber}
					placeholder="Rodné číslo"
				/>
				<span class="ml-2">
					<TextField
						error={$errors.study}
						on:change={handleChange}
						bind:value={$form.study}
						type="text"
						placeholder="Obor"
					/>
				</span>
			</div>
		{/if}
		<input
			on:click={async (e) => {
				await handleSubmit(e);
				console.log('clicked ' + isPageInvalid());
				if (isPageInvalid()) return;
				if (pageIndex === pageCount) {
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
