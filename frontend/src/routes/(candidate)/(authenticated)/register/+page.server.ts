import { apiFetchDetails } from '$lib/@api/candidate';
import type { CandidateData } from '$lib/stores/candidate';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const details: CandidateData | undefined = await apiFetchDetails(fetch).catch((e) => {
		console.error(e);
		return undefined;
	});

	return {
		candidate: details
	};
};
