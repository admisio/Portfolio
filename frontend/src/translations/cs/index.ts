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
	components: {
		checkbox: {
			accountLinkCheckBox: {
				ok: 'Vše je v pořádku',
				whatHappened: 'Co se děje?',
				multiple: {
					title:
						'Ano, podával/a jsem dvě přihlášky na dva obory SSPŠaG ({first:number}) a ({second:number})',
					title2: 'Ne, přihlášku na SSPŠaG jsem podával/a jen jednu ({first:number})'
				},
				single: {
					title: 'Ano, přihlášku na SSPŠaG jsem podával/a jen jednu ({first:number})',
					title2: '`Ne, přihlášku na SSPŠaG jsem podával více přihlášek'
				}
			},
			gdprCheckBox: {
				title: 'Souhlasím se zpracováním osobních údajů',
				description: 'Kliknutím vyjaďřujete souhlas se zpracováním osobních údajů',
				here: 'Zde'
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
