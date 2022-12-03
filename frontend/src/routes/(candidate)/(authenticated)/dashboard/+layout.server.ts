import { apiFetchDetails, apiFetchSubmissionProgress } from '$lib/@api/candidate';
import type { CandidateData } from '$lib/stores/candidate';
import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ fetch }) => {
	let details: CandidateData;
	try {
		details = await apiFetchDetails(fetch);
	} catch (e: any) {
		if (e.code === 401) {
			throw redirect(302, '/auth/login');
		} else {
			throw redirect(302, '/register');
		}
	}

	let submissionProgress;
	try {
		submissionProgress = await apiFetchSubmissionProgress(fetch);
	} catch (e) {
		console.log(e);
	}

	return {
		candidate: details,
		submission: {
			...submissionProgress
		}
	};
};