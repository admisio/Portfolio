import type { PageServerLoad } from './$types';

import { redirect } from '@sveltejs/kit';
import { apiLogout } from '$lib/@api/admin';

export const load: PageServerLoad = async ({ fetch, cookies }) => {
	const a = await apiLogout(fetch);
	console.log(a);

	cookies.delete('id', { path: '/' });
	cookies.delete('key', { path: '/' });

	throw redirect(302, '/admin/auth/login');
};
