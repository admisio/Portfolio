import { setLocale } from '$i18n/i18n-svelte';
import { loadAllLocalesAsync } from '$i18n/i18n-util.async';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ url }) => {
	await loadAllLocalesAsync();
	setLocale('cs');
	
	return {
		url: url.pathname
	};
};
