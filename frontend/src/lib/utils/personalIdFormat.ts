// TODO: nefunguje pro lidi nar. pred 1.1.1954 :D
export const isPersonalIdNumberValid = (personalIdNumber: string): boolean => {
	const idFmt = personalIdNumber.split('/').join('');

	const lastDigitCheck =
		Number(idFmt.slice(0, 9)) % 11 === Number(idFmt.at(-1)) ||
		Number(idFmt.slice(0, 9)) % 11 === 10; // an edge case that could occur
	const divisibleBy11 = Number(idFmt) % 11 === 0;

	if (lastDigitCheck && divisibleBy11) {
		return true;
	} else {
		return false;
	}
};

export const isPersonalIdMatchingBirthdate = (
	personalIdNumber: string,
	birthdate: string
): boolean => {
	const dateFmt = birthdate
		.split('.')
		.map((x) => x.padStart(2, '0'))
		.reverse()
		.join('')
		.slice(2);
	const idFmt = personalIdNumber.split('/').join('');

	const divisionValid = isPersonalIdNumberValid(personalIdNumber);

	const idMonth = Number(idFmt.slice(2, 4));
	const dateMonth = Number(dateFmt.slice(2, 4));
	const monthValid =
		idMonth === dateMonth ||
		idMonth === dateMonth + 50 ||
		idMonth === dateMonth + 20 ||
		idMonth === dateMonth + 70;

	if (
		idFmt.slice(0, 2) === dateFmt.slice(0, 2) &&
		monthValid &&
		idFmt.slice(4, 6) === dateFmt.slice(4, 6) &&
		divisionValid
	) {
		return true;
	} else {
		return false;
	}
};

export const parseBirthdateSexFromPersonalId = (
	personalIdNumber: string
): [birthdate: string, sex: 'Muž' | 'Žena'] => {
	const yearPadded = Number(personalIdNumber.slice(0, 2));
	const year = yearPadded < 24 ? yearPadded + 2000 : yearPadded + 1900;
	const idMonth = Number(personalIdNumber.slice(2, 4));
	let month;
	let sex: 'Muž' | 'Žena';
	if (idMonth > 12 && idMonth <= 32) {
		month = idMonth - 20;
		sex = 'Muž';
	} else if (idMonth > 50 && idMonth <= 52) {
		month = idMonth - 50;
		sex = 'Žena';
	} else if (idMonth > 70 && idMonth <= 82) {
		month = idMonth - 70;
		sex = 'Žena';
	} else {
		month = idMonth;
		sex = 'Muž';
	}
	const day = Number(personalIdNumber.slice(4, 6));

	const birthdate = `${day}.${month}.${year}`;
	return [birthdate, sex];
};
