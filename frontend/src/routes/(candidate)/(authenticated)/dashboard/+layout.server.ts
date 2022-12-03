import { apiFetchDetails } from '$lib/@api/candidate';
import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ fetch }) => {
	const details = await apiFetchDetails(fetch);

	if (details === null) {
		throw redirect(302, '/register');
	}
};
