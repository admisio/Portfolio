import { API_URL } from '$lib/@api';
import type {  HandleFetch } from '@sveltejs/kit';

export const handleFetch: HandleFetch = async ({ request, fetch, event,}) => {
	console.log(`SSR: handleFetch() BEFORE: ${request.method} ${request.url}`);

    const cookie = event.request.headers.get('cookie') || '';

    console.log(`SSR: handleFetch() cookie: ${cookie}`);
    
    request.headers.set('cookie', cookie);

    request = new Request(
        request.url.replace(API_URL, 'http://127.0.0.1:8000'),
        request
     );

    console.log(`SSR: handleFetch() AFTER:  ${request.method} ${request.url}`);

	return fetch(request);
};
