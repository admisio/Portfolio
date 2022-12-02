import axios, { type AxiosProgressEvent } from 'axios';
import type { CandidateData, CandidateLogin } from 'src/stores/candidate';
import { API_URL, errorHandler } from '.';

export async function apiWhoami(): Promise<string> {
	try {
		const res = await axios.get(`${API_URL}/whoami`);
		return res.data;
	} catch (e) {
		throw errorHandler(e, 'Whoami failed');
	}
}

export async function apiLogin(data: CandidateLogin): Promise<number> {
	try {
		const res = await axios.post(API_URL + '/candidate/login', data, { withCredentials: true });
		return data.applicationId;
	} catch (e: any) {
		throw errorHandler(e, 'Login failed');
	}
}

// TODO
export async function apiLogout() {
	try {
		await axios.post(API_URL + '/candidate/logout', { withCredentials: true });
	} catch (e: any) {
		throw errorHandler(e, 'Logout failed');
	}
}

export async function apiFillDetails(data: CandidateData): Promise<CandidateData> {
	console.log(data);
	try {
		const res = await axios.post(API_URL + '/candidate/details', data, { withCredentials: true });
		return res.data;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to fill details');
	}
}

export async function apiFetchDetails(): Promise<CandidateData> {
	try {
		const res = await axios.get(API_URL + '/candidate/details', { withCredentials: true });
		return res.data;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to fill details');
	}
}

export async function apiUploadCoverLetter(
	letter: File,
	progressReporter: (progress: AxiosProgressEvent) => void
): Promise<boolean> {
	try {
		const res = await axios.post(
			API_URL + '/candidate/cover_letter',
			{ letter: letter },
			{ withCredentials: true, onUploadProgress: progressReporter }
		);
		return res.data === 'true';
	} catch (e: any) {
		throw errorHandler(e, 'Failed to upload cover letter');
	}
}

export async function apiUploadPortfolioLetter(
	letter: File,
	progressReporter: (progress: AxiosProgressEvent) => void
): Promise<boolean> {
	try {
		const res = await axios.post(
			API_URL + '/candidate/portfolio_letter',
			{ letter: letter },
			{ withCredentials: true, onUploadProgress: progressReporter }
		);
		return res.data === 'true';
	} catch (e: any) {
		throw errorHandler(e, 'Failed to upload portfolio letter');
	}
}

export async function apiUploadPortfolioZip(
	portfolio: File,
	progressReporter: (progress: AxiosProgressEvent) => void
): Promise<boolean> {
	try {
		const res = await axios.post(
			API_URL + '/candidate/portfolio_zip',
			{ portfolio: portfolio },
			{ withCredentials: true, onUploadProgress: progressReporter }
		);
		return res.data === 'true';
	} catch (e: any) {
		throw errorHandler(e, 'Failed to upload portfolio zip');
	}
}
