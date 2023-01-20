import type { BaseTranslation } from '../i18n-types.js';

const cs: BaseTranslation = {
	admin: {
		auth: {
			login: {
				title: 'Přihlášení',
				description: 'Administrátoská sekce aplikace\nPřístup povolen pouze pro oprávněné osoby!'
			}
		}
	},
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
					description: 'Nevíte si rady? Klikněte',
					here: 'zde'
				}
			}
		}
	},
	input: {
		evidenceNumber: 'Ev. číslo',
		adminId: 'Admin Id',
		password: 'Heslo',
		submit: 'Odeslat'
	}
};

export default cs;
