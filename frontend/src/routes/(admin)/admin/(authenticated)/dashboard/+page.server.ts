import { apiListCandidates } from '$lib/@api/admin';
import type { CandidatePreview } from '$lib/stores/candidate';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, url }) => {
	let candidatePreview: [CandidatePreview] = [{}];
	try {
		candidatePreview = await apiListCandidates(fetch, url.searchParams);
	} catch (e) {
		console.error(e);
	}

	return {
		preview: candidatePreview,
	};
};
