import type { LayoutServerLoad } from './$types';

import { redirect } from '@sveltejs/kit';

export const load: LayoutServerLoad = ({ cookies }) => {
	const isAuthenticated = cookies.get('id');
	if (!isAuthenticated) {
		throw redirect(302, '/auth/login');
	}
};
