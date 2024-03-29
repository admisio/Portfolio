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
	import AddressField from '$lib/components/textfield/AddressField.svelte';
	import NameField from '$lib/components/textfield/NameField.svelte';
	import TelephoneField from '$lib/components/textfield/TelephoneField.svelte';
	import TextField from '$lib/components/textfield/TextField.svelte';
	import type { PageData } from './$types';
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';
	import parsePhoneNumber from 'libphonenumber-js';
	import { createForm } from 'svelte-forms-lib';
	import * as yup from 'yup';
	import type { CandidateData, SchoolJson } from '$lib/stores/candidate';
	import AccountLinkCheckBox from '$lib/components/checkbox/AccountLinkCheckBox.svelte';
	import GradesTable from '$lib/components/grades/GradesTable.svelte';
	import SchoolSelect from '$lib/components/select/SchoolSelect/SchoolSelect.svelte';
	import PersonalIdConfirmCheckBox from '$lib/components/checkbox/PersonalIdConfirmCheckBox.svelte';
	import {
		parseBirthdateSexFromPersonalId,
		isPersonalIdMatchingBirthdate
	} from '$lib/utils/personalIdFormat';
	import PersonalIdErrorModal from '$lib/components/modal/PersonalIdErrorModal.svelte';
	import LinkErrorModal from '$lib/components/modal/LinkErrorModal.svelte';
	import type { Writable } from 'svelte/store';
	import { pushErrorText, pushSuccessText } from '$lib/utils/toast';

	// import schoolList from '$lib/assets/list/school.json';
	import schoolList from '$lib/assets/list/high_schools.json';
	import countriesList from '$lib/assets/list/countries.json';

	// const schoolList = highSchoolList.map((school) => school['n']);
	const schoolNames = schoolList.map((school: SchoolJson) => school['n']);

	let pageIndex = 0;
	let pagesFilled = [false, false, false, false, false, false, false, false];
	const editModePageIndex = 3;
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
			personalIdNumber: 'TODO: remove this',
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
				.test((_val) => {
					if (!_val) return false;
					const number = parsePhoneNumber(_val);
					if (!number) return false;
					return number.isValid();
				}), // already validated by the 'TelephoneField' component
			birthplace: yup.string().required(),
			birthdate: yup
				.string()
				.required()
				.matches(/^([0-3]?[0-9])\.(0?[1-9]|1[0-2])\.[0-9]{4}$/)
				.test((_val) => {
					if ($form.candidate.citizenship !== 'Česká republika') return true;
					if (!_val) return false;
					if (isPersonalIdMatchingBirthdate(data.whoami.personalIdNumber, _val)) {
						return true;
					} else {
						pushErrorText('Datum narození a rodné číslo se neshodují.');
						return false;
					}
				}),
			birthSurname: yup.string(),
			sex: yup.string(),
			address: yup.string(),
			street: yup.string().required(),
			houseNumber: yup
				.string()
				.required()
				.matches(/^[0-9]+(\/[0-9]+([a-zA-Z]+)?)?$/),
			city: yup.string().required(),
			zip: yup.string().required(),
			letterAddress: yup.string(),
			citizenship: yup.string().required(),
			personalIdNumber: yup.string(),
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
				name: yup
					.string()
					.required()
					.test((_val) => {
						if (!_val) return false;
						if (schoolNames.includes(_val)) {
							return true;
						} else {
							pushErrorText('Vyberte prosím školu ze seznamu.');
							return false;
						}
					}),
				field: yup.string().required()
			}),
			secondSchool: yup.object().shape({
				name: yup
					.string()
					.required()
					.test((_val) => {
						if (!_val) return false;
						if (!_val) return false;
						if (schoolNames.includes(_val)) {
							return true;
						} else {
							pushErrorText('Vyberte prosím školu ze seznamu.');
							return false;
						}
					}),
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
					if (context.path.includes('parents[1]')) {
						return true;
					}
					if (!_val) return false;
					const number = parsePhoneNumber(_val);
					if (!number) return false;
					return number.isValid();
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

	let visibleModals = {
		personalIdModal: false,
		linkErrorModal: false
	};

	const onSubmit = async (values: CandidateData) => {
		if (pageIndex === pageCount) {
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
				pushErrorText('Neznámá chyba při odesílání dat.');
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
					visibleModals.personalIdModal = true;
					return true;
				}
				break;
			case 1:
				if ($typedErrors['linkOk'] || $typedErrors['linkError']) {
					visibleModals.linkErrorModal = true;
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
					$typedErrors['candidate']['zip'] ||
					$typedErrors['candidate']['letterAddress']
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
					$typedErrors['candidate']['birthSurname'] ||
					$typedErrors['candidate']['testLanguage']
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
				// @ts-ignore
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

	if (details !== undefined) {
		details.candidate.birthdate = details.candidate.birthdate.split('-').reverse().join('.');

		details.candidate.telephone = formatTelephone(details.candidate.telephone);
		details.parents.map(
			(x) => (x.telephone = x.telephone != '' ? formatTelephone(x.telephone) : '')
		);

		const addressArray = details.candidate.address.split(',');
		const streetHouseNumber = addressArray[0].split(' ');
		form.set({
			gdpr: true,
			linkOk: true,
			linkError: false,
			personalIdOk: true,
			personalIdErr: false,
			candidate: {
				...details.candidate,
				street: streetHouseNumber
					.slice(0, streetHouseNumber.length - 1)
					.join(' ')
					.trim(),
				houseNumber: streetHouseNumber[streetHouseNumber.length - 1],
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

	let lastCitizenshipSelected = $form.candidate.citizenship;
	$: if ($form.candidate.citizenship !== lastCitizenshipSelected) {
		lastCitizenshipSelected = $form.candidate.citizenship;
		$form.candidate.birthdate = '';
		$form.candidate.sex = '';

		if ($form.candidate.citizenship === 'Česká republika') {
			let [birthdate, sex] = parseBirthdateSexFromPersonalId(data.whoami.personalIdNumber);
			$form.candidate.birthdate = birthdate;
			$form.candidate.sex = sex;
			if (pageIndex === 4) {
				pushSuccessText(
					`Datum narození a pohlaví bylo vyplněno automaticky podle Vašeho rodného čísla (${data.whoami.personalIdNumber}).`
				);
			}
		}
	}
</script>

<SplitLayout>
	<SvelteToast />
	{#if visibleModals.personalIdModal}
		<PersonalIdErrorModal
			on:close={(_) => (visibleModals.personalIdModal = false)}
			personalIdNumber={baseCandidateDetails.personalIdNumber}
		/>
	{:else if visibleModals.linkErrorModal}
		<LinkErrorModal
			applications={baseCandidateDetails.applications}
			on:close={(_) => (visibleModals.linkErrorModal = false)}
		/>
	{/if}
	<div class="form relative bg-center">
		<div class="bottom-5/24 absolute flex w-full flex-col md:h-auto">
			{#if pageIndex !== 3}
				<div class="<md:hidden self-center">
					<SchoolBadge />
				</div>
			{/if}
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
							<span class="field">
								<NameField
									error={$typedErrors['candidate']['name'] || $typedErrors['candidate']['surname']}
									bind:valueName={$form.candidate.name}
									bind:valueSurname={$form.candidate.surname}
									placeholder={$LL.input.nameSurname()}
								/>
							</span>
							<span class="field">
								<TelephoneField
									bind:error={$typedErrors['candidate']['telephone']}
									bind:value={$form.candidate.telephone}
									placeholder={$LL.input.telephone()}
								/>
							</span>
							<div>
								<div class="field flex">
									<span class="w-[50%]">
										<EmailField
											error={$typedErrors['candidate']['email']}
											bind:value={$form.candidate.email}
											placeholder={$LL.input.email()}
										/>
									</span>
									<span class="ml-2 w-[50%]">
										<TextField
											error={$typedErrors['candidate']['city']}
											bind:value={$form.candidate.city}
											type="text"
											placeholder={$LL.input.city()}
											helperText="Uveďte okres / MČ Prahy (např. Liberec nebo Praha 5)"
										/>
									</span>
								</div>
							</div>
						</div>
						<div class="field flex">
							<span class="w-[66%]">
								<AddressField
									error={$typedErrors['candidate']['street'] ||
										$typedErrors['candidate']['houseNumber']}
									bind:valueLeft={$form.candidate.street}
									bind:valueRight={$form.candidate.houseNumber}
									placeholder={$LL.input.address()}
									helperText="Uveďte ulici a číslo popisné (např. Preslova 72/25)."
								/>
							</span>
							<span class="ml-2 w-[33%]">
								<TextField
									error={$typedErrors['candidate']['zip']}
									bind:value={$form.candidate.zip}
									type="text"
									placeholder={$LL.input.zipCode()}
									helperText="Uveďte poštovní směrovací číslo. (např. 150 21)"
								/>
							</span>
						</div>
						<div class="flex w-full flex-col">
							<span class="field">
								<TextField
									error={$typedErrors['candidate']['letterAddress']}
									bind:value={$form.candidate.letterAddress}
									type="text"
									placeholder={'Adresa pro doručování písemností (pokud odlišná)'}
									helperText="Uveďte adresu pro doručování písemností. Musí obsahovat <strong>ulici a č.p., PSČ, Okres</strong> <br />(např. Preslova 72/25, 150 21, Praha 5)"
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
							options={countriesList}
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
						helperText="Uveďte datum narození (např. 1. 1. 1970)"
					/>
					<div class="ml-2">
						<TextField
							error={$typedErrors['candidate']['birthplace']}
							bind:value={$form.candidate.birthplace}
							type="text"
							placeholder={$LL.input.birthPlace()}
							helperText="Uveďte místo narození (např. Liberec nebo Praha 5)"
						/>
					</div>
				</div>
				<div class="field flex items-center justify-center">
					<TextField
						error={$typedErrors['candidate']['birthSurname']}
						bind:value={$form.candidate.birthSurname}
						placeholder={`${$LL.input.birthSurname()} (${$LL.input.optional()})`}
					/>
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
								helperText="Uveďte IZO základní školy (např. 47608579)"
							/>
						{:else}
							<TextField
								error={$typedErrors['candidate']['schoolName']}
								type="text"
								bind:value={$form.candidate.schoolName}
								placeholder={$LL.input.schoolName()}
								helperText="Uveďte název základní školy (např. Masarykova základní škola, Praha 9 - Újezd nad Lesy, Polesná 1690)"
							/>
						{/if}
					</span>

					<span class="ml-2">
						<TextField
							error={$typedErrors['candidate']['healthInsurance']}
							type="text"
							bind:value={$form.candidate.healthInsurance}
							placeholder={$LL.input.insuranceNumber()}
							helperText="Uveďte číslo zdravotní pojišťovny (např. 111)"
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
							bind:error={$typedErrors['parents'][0]['telephone']}
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
							bind:error={$typedErrors['parents'][1]['telephone']}
							bind:value={$form.parents[1].telephone}
							placeholder={`${$LL.input.parent.telephone()} (${$LL.input.optional()})`}
						/>
					</span>
				</div>
			{:else if pageIndex === 7}
				<!-- <h1 class="title mt-8">{pageTexts[5]}</h1> -->
				<!-- <p class="description mt-8 block text-center">
					{$LL.candidate.register.seventh.description()}
				</p> -->
				<div class="flex h-full flex-col justify-between">
					<span class="field">
						<h2 class="text-sspsBlueDark mb-6 text-3xl font-bold">
							První škola - termín JPZ: <span class="underline">13. 4. 2023</span>
						</h2>
						<SchoolSelect
							{schoolNames}
							{schoolList}
							error={$typedErrors['candidate']['firstSchool']['name'] ||
								$typedErrors['candidate']['firstSchool']['field']}
							bind:selectedSchool={$form.candidate.firstSchool}
						/>
					</span>
					<!--dotted line -->
					<svg class="mt-12 h-[10px] w-full" viewBox="0 0 800 5">
						<line
							x1="0"
							y1="0"
							x2="100%"
							y2="0"
							stroke="black"
							stroke-width="3"
							stroke-dasharray="10"
						/>
					</svg>

					<span class="field mt-10">
						<h2 class="text-sspsBlueDark mb-6 text-3xl font-bold">
							Druhá škola - termín JPZ: <span class="underline">14. 4. 2023</span>
						</h2>
						<SchoolSelect
							{schoolNames}
							{schoolList}
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
					enterAllowed={pageIndex !== 7}
					on:click={async (e) => {
						await handleSubmit(e);
						if (isPageInvalid(pageIndex)) return;
						if (pageIndex !== pageCount) {
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
		@apply lg:w-9/10 mt-4 w-full md:mt-8 lg:mx-auto 2xl:w-4/5;
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
