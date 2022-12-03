import type { PageServerLoad } from './$types';

import { redirect } from '@sveltejs/kit';
import { logout } from '$lib/stores/candidate';

export const load: PageServerLoad = async ({ cookies }) => {
	// TODO: Nefunguje?!
	await logout();
    console.log(cookies);
	cookies.delete('id', {path: '/'});
	cookies.delete('key', {path: '/'});
	throw redirect(302, '/auth/login');
};
