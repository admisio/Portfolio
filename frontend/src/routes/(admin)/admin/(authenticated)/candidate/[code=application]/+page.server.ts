import { apiFetchCandidate } from '$lib/@api/admin';
import type { CandidateData } from '$lib/stores/candidate';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const { code } = params;
	const codeNumber = Number(code);

	let candidateData: CandidateData = {
		candidate: {
			name: '',
			surname: '',
			birthplace: '',
			birthdate: '',
			address: '',
			telephone: '',
			citizenship: '',
			email: '',
			sex: '',
			study: '',
			personalIdNumber: ''
		},
		parents: []
	};
	try {
		candidateData = await apiFetchCandidate(codeNumber, fetch);
	} catch (e) {
		console.error(e);
	}

	return {
		id: codeNumber,
		candidateData: candidateData
	};
};
