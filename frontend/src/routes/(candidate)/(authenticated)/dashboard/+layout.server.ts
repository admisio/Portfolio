import { apiFetchDetails, apiFetchSubmissionProgress } from '$lib/@api/candidate';
import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ fetch }) => {
	let details;
	try {
		details = await apiFetchDetails(fetch);
	} catch {
		throw redirect(302, '/register');
	}

	let submissionProgress;
	try {
		submissionProgress = await apiFetchSubmissionProgress(fetch);
	} catch (e) {
		console.log(e);
	}

	return {
		candidate: {
			name: details.name,
			surname: details.surname,
			email: details.email
		},
		submission: {
			...submissionProgress
		}
	};
};
