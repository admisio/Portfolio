import type { HandleFetch } from '@sveltejs/kit';
import * as dotenv from 'dotenv';

export const handleFetch: HandleFetch = async ({ request, fetch, event }) => {
	dotenv.config();
	const cookie = event.request.headers.get('cookie') || '';


	request.headers.set('cookie', cookie);

	const url = new URL(request.url);

	url.protocol = "http:";

	url.host = process.env.PORTFOLIO_API_HOST ?? '127.0.0.1:8000';

	url.pathname = url.pathname.replace(/^\/api/, '');

	request = new Request(url, request);

	return fetch(request);
};
