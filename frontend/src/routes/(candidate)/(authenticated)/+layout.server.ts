import type { LayoutServerLoad } from './$types';

import { redirect } from '@sveltejs/kit';
import { apiWhoami } from '$lib/@api/candidate';

export const load: LayoutServerLoad = async ({ cookies, fetch }) => {
	const isAuthenticated = cookies.get('id');

	if (isAuthenticated) {
		const whoami = await apiWhoami(fetch).catch(() => {
			throw redirect(302, '/auth/logout');
		});
		return {
			whoami: whoami
		};
	} else {
		throw redirect(302, '/auth/logout');
	}
};
