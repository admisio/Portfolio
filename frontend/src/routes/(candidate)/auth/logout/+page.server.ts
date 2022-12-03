import type { PageServerLoad } from './$types';

import { redirect } from '@sveltejs/kit';
import { apiLogout } from '$lib/@api/candidate';

export const load: PageServerLoad = async ({ fetch, cookies }) => {
	await apiLogout(fetch);

	cookies.delete('id', { path: '/' });
	cookies.delete('key', { path: '/' });

	throw redirect(302, '/auth/login');
};
