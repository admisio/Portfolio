import { apiFetchDetails, apiFetchSubmissionProgress } from '$lib/@api/candidate';
import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ fetch }) => {
	const details = await apiFetchDetails(fetch);

	if (details === null) {
		throw redirect(302, '/register');
	}

	const submissionProgress = await apiFetchSubmissionProgress(fetch);

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
