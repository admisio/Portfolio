<script lang="ts">
	import LL from '$i18n/i18n-svelte';

	import { goto } from '$app/navigation';
	import { apiFillDetails } from '$lib/@api/candidate';
	import Submit from '$lib/components/button/Submit.svelte';
	import GdprCheckBox from '$lib/components/checkbox/GdprCheckBox.svelte';

	import SchoolBadge from '$lib/components/icons/SchoolBadge.svelte';
	import SplitLayout from '$lib/components/layout/SplitLayout.svelte';
	import SelectField from '$lib/components/select/SelectField.svelte';
	import EmailField from '$lib/components/textfield/EmailField.svelte';
	import IdField from '$lib/components/textfield/IdField.svelte';
	import NameField from '$lib/components/textfield/NameField.svelte';
	import TelephoneField from '$lib/components/textfield/TelephoneField.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';
	import type { PageData } from './$types';
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';

	import { createForm } from 'svelte-forms-lib';
	import type { Writable } from 'svelte/store';
	import * as yup from 'yup';
	import type { CandidateData } from '$lib/stores/candidate';
	import AccountLinkCheckBox from '$lib/components/checkbox/AccountLinkCheckBox.svelte';
	import GradesTable from '$lib/components/grades/GradesTable.svelte';
	import SchoolSelect from '$lib/components/select/SchoolSelect/SchoolSelect.svelte';
	import PersonalIdConfirmCheckBox from '$lib/components/checkbox/PersonalIdConfirmCheckBox.svelte';
	import { isPersonalIdNumberWithBirthdateValid } from '$lib/utils/personalIdFormat';

	let pageIndex = 0;
	let pagesFilled = [false, false, false, false, false, false, false, false];
	let editModePageIndex = 3;
	const pageCount = pagesFilled.length;
	let pageTexts = [
		$LL.candidate.register.second.title(),
		$LL.candidate.register.third.title(),
		$LL.candidate.register.fourth.title(),
		$LL.candidate.register.fifth.title(),
		$LL.candidate.register.sixth.title(),
		$LL.candidate.register.seventh.title(),
		$LL.candidate.register.eighth.title()
	];

	export let data: PageData;
	let details = data.candidate;
	let baseCandidateDetails = data.whoami;

	let personalIdBirthdateMatch = true;
	const formInitialValues = {
		gdpr: false,
		personalIdOk: false,
		personalIdErr: false,
		linkOk: false,
		linkError: false,
		candidate: {
			name: '',
			surname: '',
			birthSurname: '',
			email: '',
			telephone: '',
			birthplace: '',
			birthdate: '',
			sex: '',
			address: '',
			letterAddress: '',
			street: '',
			houseNumber: '',
			city: '',
			zip: '',
			citizenship: '',
			personalIdNumber: '',
			schoolName: '',
			healthInsurance: '',
			grades: [],
			firstSchool: { name: '', field: '' },
			secondSchool: { name: '', field: '' },
			testLanguage: ''
		},
		parents: [
			{
				name: '',
				surname: '',
				email: '',
				telephone: ''
			},
			{
				name: '',
				surname: '',
				email: '',
				telephone: ''
			}
		]
	};

	const formValidationSchema = yup.object().shape({
		gdpr: yup.boolean().oneOf([true]),
		personalIdOk: yup.boolean().oneOf([true]),
		personalIdErr: yup.boolean().oneOf([false]),
		linkOk: yup.boolean().oneOf([true]),
		linkError: yup.boolean().oneOf([false]),
		candidate: yup.object().shape({
			name: yup.string().required(),
			surname: yup.string().required(),
			email: yup.string().email().required(),
			telephone: yup
				.string()
				.required()
				.matches(/^\+\d{1,3} \d{3} \d{3} \d{3}$/),
			birthplace: yup.string().required(),
			birthdate: yup
				.string()
				.required()
				.matches(/^([0-3]?[0-9])\.(0?[1-9]|1[0-2])\.[0-9]{4}$/),
			sex: yup.string(),
			address: yup.string(),
			street: yup.string().required(),
			houseNumber: yup
				.string()
				.required()
				.matches(/^[0-9]+(\/[0-9]+)?$/),
			city: yup.string().required(),
			zip: yup.string().required(),
			citizenship: yup.string().required(),
			personalIdNumber: yup.string().required(),
			schoolName: yup.string().required(),
			healthInsurance: yup.number().required(),
			grades: yup
				.array()
				.min(1)
				.of(
					yup
						.object()
						.shape({
							subject: yup.string().required(),
							value: yup.number().required(),
							semester: yup.string().required()
						})
						.required()
				)
				.required(),
			firstSchool: yup.object().shape({
				name: yup.string().required(),
				field: yup.string().required()
			}),
			secondSchool: yup.object().shape({
				name: yup.string().required(),
				field: yup.string().required()
			}),
			testLanguage: yup.string().required()
		}),
		parents: yup.array().of(
			yup.object().shape({
				name: yup.string().test((_val, context) => {
					if (context.path.includes('parents[1]')) {
						return true;
					}
					return _val !== '';
				}),
				surname: yup.string().test((_val, context) => {
					if (context.path.includes('parents[1]')) {
						return true;
					}
					return _val !== '';
				}),
				email: yup
					.string()
					.email()
					.test((_val, context) => {
						if (context.path.includes('parents[1]') && _val === '') {
							return true;
						}
						return _val !== '';
					}),
				telephone: yup.string().test((_val, context) => {
					if (context.path.includes('parents[1]') && _val === '') {
						return true;
					}
					return _val?.match(/^\+\d{1,3} \d{3} \d{3} \d{3}$/) !== null;
				})
			})
		)
	});

	type FormErrorType = {
		[K in keyof typeof formInitialValues]: (typeof formInitialValues)[K] extends Record<
			string,
			unknown
		>
			? {
					[K2 in keyof (typeof formInitialValues)[K]]: (typeof formInitialValues)[K][K2] extends Record<
						string,
						unknown
					>
						? { [K4 in keyof (typeof formInitialValues)[K][K2]]: string }
						: string;
			  }
			: (typeof formInitialValues)[K] extends Array<Record<string, unknown>>
			? Array<{ [K3 in keyof (typeof formInitialValues)[K][number]]: string }>
			: string;
	};

	// TODO: https://github.com/tjinauyeung/svelte-forms-lib/issues/171!! (Zatím tenhle mega typ)
	$: typedErrors = errors as unknown as Writable<FormErrorType>;

	$: console.log($typedErrors);
	const onSubmit = async (values: CandidateData) => {
		if (pageIndex === 3) {
			if (values.candidate.citizenship === 'Česká republika') {
				if (
					!isPersonalIdNumberWithBirthdateValid(
						values.candidate.personalIdNumber,
						values.candidate.birthdate
					)
				) {
					toast.push('Rodné číslo neodpovídá oficiální specifikaci či datumu narození', {
						theme: {
							'--toastColor': 'mintcream',
							'--toastBackground': '#b91c1c',
							'--toastBarBackground': '#7f1d1d'
						}
					});
					personalIdBirthdateMatch = false;
					throw new Error('Rodné číslo neodpovídá datumu narození');
				}
			}
			personalIdBirthdateMatch = true;
		}
		if (pageIndex === pageCount) {
			console.log('submitting');
			// clone values to oldValues
			let oldValues = JSON.parse(JSON.stringify(values));
			try {
				// @ts-ignore // love javascript
				delete values.undefined;
				// convert birthdate from dd.mm.yyyy to yyyy-mm-dd
				let birthdate_formttted = values.candidate
					.birthdate!.split('.')
					.map((x) => x.padStart(2, '0'))
					.reverse()
					.join('-');

				values.candidate.birthdate = birthdate_formttted;

				values.parents = values.parents.filter(
					(x) => x.name !== '' && x.surname !== '' && x.email !== '' && x.telephone !== ''
				);
				let addressArray: Array<string> = [
					// @ts-ignore
					values.candidate.street + ' ' + values.candidate.houseNumber,
					// @ts-ignore
					values.candidate.city,
					// @ts-ignore
					values.candidate.zip
				];
				values.candidate.address = addressArray.map((x) => x.replaceAll(',', '').trim()).join(',');
				// @ts-ignore
				delete values.candidate.street;
				// @ts-ignore
				delete values.candidate.houseNumber;
				// @ts-ignore
				delete values.candidate.city;
				// @ts-ignore
				delete values.candidate.zip;

				await apiFillDetails(values);

				goto('/dashboard');
			} catch (e) {
				values = oldValues;
				$form = oldValues;
				console.error('error while submitting data: ' + e);
			}
		}
	};

	const { form, errors, handleSubmit } = createForm({
		initialValues: formInitialValues,
		validationSchema: formValidationSchema,

		onSubmit: async (values: CandidateData) => onSubmit(values)
	});

	const isPageInvalid = (index: number): boolean => {
		switch (index) {
			case 0:
				if ($typedErrors['personalIdOk'] || $typedErrors['personalIdErr']) {
					return true;
				}
				break;
			case 1:
				if ($typedErrors['linkOk'] || $typedErrors['linkError']) {
					return true;
				}
				break;
			case 2:
				if ($typedErrors['gdpr']) {
					return true;
				}
				break;
			case 3:
				if (
					$typedErrors['candidate']['name'] ||
					$typedErrors['candidate']['surname'] ||
					$typedErrors['candidate']['email'] ||
					$typedErrors['candidate']['telephone'] ||
					$typedErrors['candidate']['city'] ||
					$typedErrors['candidate']['street'] ||
					$typedErrors['candidate']['houseNumber'] ||
					$typedErrors['candidate']['zip']
				) {
					return true;
				}
				break;

			case 4:
				if (
					$typedErrors['candidate']['citizenship'] ||
					$typedErrors['candidate']['personalIdNumber'] ||
					$typedErrors['candidate']['schoolName'] ||
					$typedErrors['candidate']['healthInsurance'] ||
					$typedErrors['candidate']['birthdate'] ||
					$typedErrors['candidate']['birthplace'] ||
					$typedErrors['candidate']['personalIdNumber'] ||
					$typedErrors['candidate']['testLanguage'] ||
					!personalIdBirthdateMatch
				) {
					return true;
				}
				break;
			case 5:
				if (
					$typedErrors['parents'][0]['name'] ||
					$typedErrors['parents'][0]['surname'] ||
					$typedErrors['parents'][0]['email'] ||
					$typedErrors['parents'][0]['telephone']
				) {
					return true;
				}
				break;
			case 6:
				if (
					$typedErrors['parents'][1]['name'] ||
					$typedErrors['parents'][1]['surname'] ||
					$typedErrors['parents'][1]['email'] ||
					$typedErrors['parents'][1]['telephone']
				) {
					return true;
				}
				break;
			case 7:
				if (
					$typedErrors['candidate']['firstSchool']['name'] ||
					$typedErrors['candidate']['firstSchool']['field'] ||
					$typedErrors['candidate']['secondSchool']['name'] ||
					$typedErrors['candidate']['secondSchool']['field']
				) {
					return true;
				}
				break;
			case 8:
				if ($typedErrors['candidate']['grades'].length > 0) return true;
				break;
			default:
				return false;
		}
		return false;
	};

	const formatTelephone = (telephone: string) => {
		return '+' + telephone.match(/[0-9]{1,3}/g)!.join(' ');
	};

	/* $form.candidate.personalIdNumber = data.whoami.personalIdNumber;
	const [birthdate, sex] = deriveBirthdateFromPersonalId(data.whoami.personalIdNumber);
	$form.candidate.birthdate = birthdate;
	$form.candidate.sex = sex; */

	if (details !== undefined) {
		details.candidate.birthdate = details.candidate.birthdate.split('-').reverse().join('.');

		details.candidate.telephone = formatTelephone(details.candidate.telephone);
		details.parents.map(
			(x) => (x.telephone = x.telephone != '' ? formatTelephone(x.telephone) : '')
		);
		form.set({
			gdpr: true,
			linkOk: true,
			linkError: false,
			personalIdOk: true,
			personalIdErr: false,
			candidate: {
				...details.candidate,
				street: details.candidate.address.split(',')[0].split(' ')[0],
				houseNumber: details.candidate.address.split(',')[0].split(' ')[1],
				city: details.candidate.address.split(',')[1],
				zip: details.candidate.address.split(',')[2],
				// @ts-ignore
				grades: details.candidate.grades
			},
			parents: [
				{
					...details.parents[0]
				},
				{
					...(details.parents[1] ?? {
						name: '',
						surname: '',
						email: '',
						telephone: ''
					})
				}
			]
		});
		pageIndex = editModePageIndex; // skip gdpr page
		pageTexts[2] = $LL.candidate.register.fourth.titleEdit();
	}
</script>

<SplitLayout>
	<SvelteToast />
	<div class="form relative bg-center">
		<div class="bottom-5/24 absolute flex w-full flex-col md:h-auto">
			<div class="<md:hidden self-center">
				<SchoolBadge />
			</div>
			<form on:submit={handleSubmit} id="triggerForm" class="invisible hidden" />
			{#if pageIndex === 0}
				<form on:submit={handleSubmit}>
					<h1 class="title mt-8">{$LL.candidate.register.first.title()}</h1>
					<p class="description mt-8 block text-center">
						{$LL.candidate.register.first.description()}
					</p>
					<div class="field">
						<PersonalIdConfirmCheckBox
							personalIdNumber={baseCandidateDetails.personalIdNumber}
							bind:personalIdOk={$form.personalIdOk}
							bind:personalIdErr={$form.personalIdErr}
							error={$typedErrors['personalIdOk']}
						/>
					</div>
				</form>
			{:else if pageIndex === 1}
				<form on:submit={handleSubmit}>
					<h1 class="title mt-8">{$LL.candidate.register.first.title()}</h1>
					<p class="description mt-8 block text-center">
						{$LL.candidate.register.first.description()}
					</p>
					<div class="field">
						<AccountLinkCheckBox
							applications={baseCandidateDetails.applications}
							bind:linkOk={$form.linkOk}
							bind:linkError={$form.linkError}
							error={$typedErrors['linkOk']}
						/>
					</div>
				</form>
			{:else if pageIndex === 2}
				<form on:submit={handleSubmit}>
					<h1 class="title mt-8">{pageTexts[0]}</h1>
					<p class="description mt-8 block text-center">
						{$LL.candidate.register.second.description()}
						<a href="/bezpecnost" class="text-sspsBlue underline"> {$LL.here()}</a>.
					</p>
					<div class="field">
						<GdprCheckBox bind:value={$form.gdpr} error={$typedErrors['gdpr']} />
					</div>
				</form>
			{:else if pageIndex === 3}
				<form on:submit={handleSubmit}>
					<h1 class="title mt-8">{pageTexts[1]}</h1>
					<p class="description mt-8 block text-center">
						{$LL.candidate.register.third.description()}
					</p>
					<div class="w-full">
						<div class="flex flex-col">
							<div class="field flex">
								<span class="w-[50%]">
									<NameField
										error={$typedErrors['candidate']['name'] ||
											$typedErrors['candidate']['surname']}
										bind:valueName={$form.candidate.name}
										bind:valueSurname={$form.candidate.surname}
										placeholder={$LL.input.nameSurname()}
									/>
								</span>
								<span class="ml-2 w-[50%]">
									<TextField
										error={$typedErrors['candidate']['birthSurname']}
										bind:value={$form.candidate.birthSurname}
										placeholder={$LL.input.birthSurname()}
									/>
								</span>
							</div>
							<div class="field flex">
								<span class="w-[50%]">
									<EmailField
										error={$typedErrors['candidate']['email']}
										bind:value={$form.candidate.email}
										placeholder={$LL.input.email()}
									/>
								</span>
								<span class="ml-2 w-[50%]">
									<TelephoneField
										error={$typedErrors['candidate']['telephone']}
										bind:value={$form.candidate.telephone}
										placeholder={$LL.input.telephone()}
									/>
								</span>
							</div>
							<span class="field">
								<TextField
									error={$typedErrors['candidate']['city']}
									bind:value={$form.candidate.city}
									type="text"
									placeholder={$LL.input.city()}
									helperText="Uveďte poštovní směrovací číslo. (např. 602 00)"
								/>
							</span>
						</div>
						<div class="field flex">
							<span class="w-[66%]">
								<NameField
									error={$typedErrors['candidate']['street'] ||
										$typedErrors['candidate']['houseNumber']}
									bind:valueName={$form.candidate.street}
									bind:valueSurname={$form.candidate.houseNumber}
									placeholder={$LL.input.address()}
									helperText="Uveďte ulici a číslo popisné (např. Preslova 72)."
								/>
							</span>
							<span class="ml-2 w-[33%]">
								<TextField
									error={$typedErrors['candidate']['zip']}
									bind:value={$form.candidate.zip}
									type="number"
									placeholder={$LL.input.zipCode()}
									helperText="Uveďte poštovní směrovací číslo. (např. 602 00)"
								/>
							</span>
						</div>
					</div>
				</form>
			{:else if pageIndex === 4}
				<h1 class="title mt-8">{pageTexts[2]}</h1>
				<p class="description mt-8 block text-center">
					{$LL.candidate.register.fourth.description()}
				</p>
				<div class="field flex w-full">
					<span class="w-[50%]">
						<SelectField
							error={$typedErrors['candidate']['citizenship']}
							bind:value={$form.candidate.citizenship}
							placeholder={$LL.input.citizenship()}
							options={['Česká republika', 'Slovenská republika', 'Ukrajina', 'Jiné']}
						/>
					</span>
					<span class="ml-2 w-[50%]">
						<SelectField
							error={$typedErrors['candidate']['testLanguage']}
							bind:value={$form.candidate.testLanguage}
							placeholder={$LL.input.testLanguage()}
							options={['Čeština', 'Angličtina']}
						/>
					</span>
				</div>
				<div class="field flex items-center">
					<TextField
						error={$typedErrors['candidate']['birthdate']}
						bind:value={$form.candidate.birthdate}
						type="text"
						placeholder={$LL.input.birthDate()}
						helperText="TODO: (Uveďte ve formátu DD.MM.RRRR)"
					/>
					<div class="ml-2">
						<TextField
							error={$typedErrors['candidate']['birthplace']}
							bind:value={$form.candidate.birthplace}
							type="text"
							placeholder={$LL.input.birthPlace()}
							helperText="TODO: (Místo narození)"
						/>
					</div>
				</div>
				<div class="field flex items-center justify-center">
					{#if $form.candidate.citizenship === 'Česká republika' || !$form.candidate.citizenship}
						<IdField
							error={$typedErrors['candidate']['personalIdNumber']}
							bind:value={$form.candidate.personalIdNumber}
							placeholder={$LL.input.personalIdentificationNumber()}
						/>
					{:else}
						<TextField
							error={$typedErrors['candidate']['personalIdNumber']}
							bind:value={$form.candidate.personalIdNumber}
							placeholder={$LL.input.personalIdentificationNumber()}
						/>
					{/if}
					<div class="ml-2">
						<SelectField
							error={$typedErrors['candidate']['sex']}
							bind:value={$form.candidate.sex}
							options={['Žena', 'Muž']}
							placeholder={$LL.input.sex()}
						/>
					</div>
				</div>
				<div class="field flex flex-row">
					<span>
						{#if $form.candidate.citizenship === 'Česká republika' || !$form.candidate.citizenship}
							<TextField
								error={$typedErrors['candidate']['schoolName']}
								type="number"
								bind:value={$form.candidate.schoolName}
								placeholder={$LL.input.schoolIzo()}
							/>
						{:else}
							<TextField
								error={$typedErrors['candidate']['schoolName']}
								type="text"
								bind:value={$form.candidate.schoolName}
								placeholder={$LL.input.schoolName()}
							/>
						{/if}
					</span>

					<span class="ml-2">
						<TextField
							error={$typedErrors['candidate']['healthInsurance']}
							type="text"
							bind:value={$form.candidate.healthInsurance}
							placeholder={$LL.input.insuranceNumber()}
						/>
					</span>
				</div>
			{:else if pageIndex === 5}
				<h1 class="title mt-8">{pageTexts[3]}</h1>
				<p class="description mt-8 block text-center">
					{$LL.candidate.register.fifth.description()}
				</p>
				<div class="flex w-full flex-col">
					<span class="field">
						<NameField
							error={$typedErrors['parents'][0]['name'] || $typedErrors['parents'][0]['surname']}
							bind:valueName={$form.parents[0].name}
							bind:valueSurname={$form.parents[0].surname}
							placeholder={$LL.input.parent.nameSurname()}
						/>
					</span>
					<span class="field">
						<EmailField
							error={$typedErrors['parents'][0]['email']}
							bind:value={$form.parents[0].email}
							placeholder={$LL.input.parent.email()}
						/>
					</span>
					<span class="field">
						<TelephoneField
							error={$typedErrors['parents'][0]['telephone']}
							bind:value={$form.parents[0].telephone}
							placeholder={$LL.input.parent.telephone()}
						/>
					</span>
				</div>
			{:else if pageIndex === 6}
				<h1 class="title mt-8">{pageTexts[4]}</h1>
				<p class="description mt-8 block text-center">
					{$LL.candidate.register.sixth.description()}
				</p>
				<div class="flex w-full flex-col">
					<span class="field">
						<NameField
							error={$typedErrors['parents'][1]['name'] || $typedErrors['parents'][1]['surname']}
							bind:valueName={$form.parents[1].name}
							bind:valueSurname={$form.parents[1].surname}
							placeholder={`${$LL.input.parent.nameSurname()} (${$LL.input.optional()})`}
						/>
					</span>
					<span class="field">
						<EmailField
							error={$typedErrors['parents'][1]['email']}
							bind:value={$form.parents[1].email}
							placeholder={`${$LL.input.parent.email()} (${$LL.input.optional()})`}
						/>
					</span>
					<span class="field">
						<TelephoneField
							error={$typedErrors['parents'][1]['telephone']}
							bind:value={$form.parents[1].telephone}
							placeholder={`${$LL.input.parent.telephone()} (${$LL.input.optional()})`}
						/>
					</span>
				</div>
			{:else if pageIndex === 7}
				<h1 class="title mt-8">Přihlášky na školy</h1>
				<div class="flex h-full flex-col justify-between">
					<span>
						<SchoolSelect
							error={$typedErrors['candidate']['firstSchool']['name'] ||
								$typedErrors['candidate']['firstSchool']['field']}
							bind:selectedSchool={$form.candidate.firstSchool}
						/>
					</span>
					<span class="mt-10 w-full">
						<SchoolSelect
							error={$typedErrors['candidate']['secondSchool']['name'] ||
								$typedErrors['candidate']['secondSchool']['field']}
							bind:selectedSchool={$form.candidate.secondSchool}
						/>
					</span>
				</div>
			{:else if pageIndex === 8}
				<h1 class="title mt-8">{pageTexts[6]}</h1>
				<p class="description mt-8 block text-center">
					{$LL.candidate.register.eighth.description()}
				</p>
				<GradesTable
					error={$typedErrors['candidate']['grades']}
					bind:grades={$form.candidate.grades}
				/>
			{/if}
		</div>
		<div class="bottom-1/24 absolute w-full">
			<div class="field">
				<Submit
					on:click={async (e) => {
						await handleSubmit(e);
						if (isPageInvalid(pageIndex)) return;
						if (pageIndex === pageCount) {
						} else {
							pagesFilled[pageIndex] = true;
							pageIndex++;
						}
						// @ts-ignore
						errors.set(formInitialValues);
					}}
					value={pageIndex === pageCount ? $LL.input.submit() : $LL.input.continue()}
				/>
			</div>

			<div class="mt-4 flex flex-row justify-center md:mt-6">
				{#each Array(pageCount + 1) as _, i}
					<button
						class:dotActive={i === pageIndex}
						on:click={async (e) => {
							pageIndex -= pageIndex === pageCount ? 1 : 0;
							await handleSubmit(e);
							pagesFilled = pagesFilled.map((_, i) => !isPageInvalid(i));

							const progress = pagesFilled.slice(0, i).every((item) => item === true);
							if (progress) {
								pageIndex = i;
							}
						}}
						class="dot"
					/>
				{/each}
			</div>
		</div>
	</div>
</SplitLayout>

<style lang="postcss">
	.field {
		@apply mt-4 w-full md:mt-8 lg:mx-auto lg:w-4/5;
	}
	.form {
		@apply flex flex-col;
		@apply mx-auto h-full w-[90%];
		@apply items-center justify-center;
	}
	.dot {
		@apply @apply hover:bg-sspsBlue @apply 
		bg-sspsGray ml-2 h-4
		w-4 rounded-full hover:cursor-pointer;
	}
	.dotActive {
		@apply bg-sspsBlue;
	}
	.description {
		@apply text-gray-500;
	}
	.title {
		@apply text-sspsBlue text-center text-4xl font-semibold;
	}
</style>
