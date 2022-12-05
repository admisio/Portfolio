import type { LayoutServerLoad } from './$types';

import { redirect } from '@sveltejs/kit';
import { apiWhoami } from '$lib/@api/candidate';

export const load: LayoutServerLoad = async ({ cookies, fetch }) => {
	const isAuthenticated = cookies.get('id');

	if (isAuthenticated) {
		await apiWhoami(fetch).catch((e) => {
			throw redirect(302, '/auth/logout');
		});
	} else {
		throw redirect(302, '/auth/logout');
	}
};
