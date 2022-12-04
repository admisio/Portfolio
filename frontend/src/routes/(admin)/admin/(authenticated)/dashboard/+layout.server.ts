import { apiListCandidates } from '$lib/@api/admin';
import type { CandidatePreview } from '$lib/stores/candidate';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ fetch, url }) => {
	const field = url.searchParams.get('field');
	let candidatePreview: [CandidatePreview] = [{}];
	try {
		candidatePreview = await apiListCandidates(fetch, field ?? undefined);
	} catch (e) {
		console.error(e);
	}

	return {
		preview: candidatePreview,
	};
};
