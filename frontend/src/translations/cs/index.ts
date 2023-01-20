import type { BaseTranslation } from '../i18n-types.js';

const cs: BaseTranslation = {
	candidate: {
		auth: {
			login: {
				title: 'Přihlášení',
				description:
					'Evidenční číslo je jedinečné číslo přidělené uchazeči, které slouží k jeho identifikaci\napřihlášení se do systému.'
			},
			application: {
				title: 'Zadejte 12místný kód pro aktivaci účtu',
				help: {
					description: "Nevíte si rady? Klikněte",
					here: 'zde'
				}
			}
		}
	},
	input: {
		evidenceNumber: 'Ev. číslo',
		submit: 'Odeslat'
	}
};

export default cs;
