import { apiFetchCandidate } from '$lib/@api/admin';
import type { CandidateData } from '$lib/stores/candidate';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const { code } = params;
	const codeNumber = Number(code);

	let candidateData: CandidateData = {};
	try {
		candidateData = await apiFetchCandidate(codeNumber, fetch);
	} catch (e) {
		console.error(e);
	}

	return {
		id: codeNumber,
		candidate: candidateData
	};
};
