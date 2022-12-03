import type {  HandleFetch } from '@sveltejs/kit';

export const handleFetch: HandleFetch = async ({ request, fetch, event,}) => {
	console.log(`SSR: handleFetch() BEFORE: ${request.method} ${request.url}`);

    const cookie = event.request.headers.get('cookie') || '';

    console.log(`SSR: handleFetch() cookie: ${cookie}`);

    request.headers.set('cookie', cookie);

	request.headers.append('Origin', event.url.origin);

    console.log(`SSR: handleFetch() AFTER:  ${request.method} ${request.url}`);

	return fetch(request);
};
