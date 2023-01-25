import type { BaseTranslation } from '../i18n-types.js';

const cs: BaseTranslation = {
	here: 'zde',
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
		},
		register: {
			first: {
				title: 'Propojení účtů',
				description:
					'Elektronickou přihlášky stačí vyplnit jen jednou i v případě, že jste podali dvě přihlášky. Potvrďte, že jste jste k nám skutečně podali dvě přihlášky.'
			},
			second: {
				title: 'Zpracování osobních údajů',
				description:
					'V rámci portálu pro přijímací řízení zpracováváme mnoho osobních údajů. Proto je nutný Váš souhlas s jejich zpracováním. O bezpečnosti zpracování Vašich osobních údajů si můžete přečíst'
			},
			third: {
				title: 'Registrace',
				description:
					'V rámci usnadnění přijímacího řízení jsme připravili online formulář, který Vám pomůže s vyplněním potřebných údajů.'
			},
			fourth: {
				title: 'Něco o Vás',
				titleEdit: 'Úprava osobních údajů',
				description:
					'Pro registraci je potřeba vyplnit několik údajů o Vás. Tyto údaje budou použity pro přijímací řízení. Všechny údaje jsou důležité.'
			},
			fifth: {
				title: 'Zákonný zástupce',
				description:
					'Sběr dat o zákonném zástupci je klíčový pro získání důležitých kontaktů a informací.'
			},
			sixth: {
				title: 'Druhý zákonný zástupce',
				description:
					'Zde můžete zadat údaje o druhém zákonném zástupci. Škole tím umožníte lépe komunikovat.'
			},
			seventh: {
				title: 'Dokončení registrace',
				description:
					'Zadejte prosím své občanství, rodné číslo, či jeho alternativu Vaší země a obor na který se hlásíte.'
			},
			eighth: {
				title: 'Poslední krok',
				description: 'Přidejte prosím přepis Vaších známek z posledních dvou let studia'
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
					title2: 'Ne, přihlášku na SSPŠaG jsem podával více přihlášek'
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
		optional: 'nepovinné',
		nameSurname: 'Jméno a příjmení',
		email: 'E-mail',
		telephone: 'Telefon',
		address: 'Ulice a č. p.',
		zipCode: 'PSČ',
		city: 'Město',
		birthPlace: 'Místo narození',
		birthDate: 'Datum narození',
		sex: 'Pohlaví',
		citizenship: 'Občanství',
		testLanguage: 'Jazyk odborných testů',
		schoolIzo: 'IZO školy',
		schoolName: 'Název školy',
		insuranceNumber: 'Číslo zdravotní pojišťovny',
		personalIdentificationNumber: 'Rodné číslo',
		evidenceNumber: 'Ev. číslo',
		adminId: 'Admin Id',
		password: 'Heslo',
		submit: 'Odeslat',
		continue: "Pokračovat",
		parent: {
			nameSurname: 'Jméno a příjmení zákonného zástupce',
			email: 'E-mail zákonného zástupce',
			telephone: 'Telefon zákonného zástupce'
		}
	}
};

export default cs;
