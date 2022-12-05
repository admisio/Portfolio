import { apiFetchDetails, apiFetchSubmissionProgress } from '$lib/@api/candidate';
import type { CandidateData } from '$lib/stores/candidate';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const details: CandidateData = await apiFetchDetails(fetch).catch((e) => {
		console.error(e);
		throw redirect(302, '/register');
	});

	const submissionProgress = await apiFetchSubmissionProgress(fetch).catch((e) => {
		console.log(e);
	});

	return {
		candidate: details,
		submission: {
			...submissionProgress
		}
	};
};
