import axios, { type AxiosProgressEvent } from 'axios';
import type { CandidateData, CandidateLogin } from '$lib/stores/candidate';
import type { SubmissionProgress } from '$lib/stores/portfolio';
import { API_URL, errorHandler } from '.';


// SSR Compatible
export const apiLogout = async (fetchSsr?: any) => {
	try {
		fetchSsr ? await fetchSsr(API_URL + '/candidate/logout', { method: 'POST', credentials: 'include' }) : await axios.post(API_URL + '/candidate/logout', { withCredentials: true });
	} catch (e: any) {
		throw errorHandler(e, 'Logout failed');
	}
}

// SSR Compatible
export const apiFetchDetails = async (fetchSsr?: any): Promise<CandidateData | null> => {
	try {
		if (fetchSsr) {
			const res = await fetchSsr(API_URL + '/candidate/details', { method: "GET", credentials: 'include' });
			const body = await res.text();
			console.log(body);
			if (res.status === 500) {
				return null;
			}
			return JSON.parse(body);
		}
		const res = await axios.get(API_URL + '/candidate/details', { withCredentials: true });
		return res.data;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to fill details');
	}
}

export const apiWhoami = async (): Promise<string> => {
	try {
		const res = await axios.get(`${API_URL}/whoami`);
		return res.data;
	} catch (e: any) {
		throw errorHandler(e, 'Whoami failed');
	}
}

export const apiLogin = async (data: CandidateLogin): Promise<number> => {
	try {
		const res = await axios.post(API_URL + '/candidate/login', data, { withCredentials: true });
		return data.applicationId;
	} catch (e: any) {
		throw errorHandler(e, 'Login failed');
	}
}


export const apiFillDetails = async (data: CandidateData): Promise<CandidateData> => {
	console.log(data);
	try {
		const res = await axios.post(API_URL + '/candidate/details', data, { withCredentials: true });
		return res.data;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to fill details');
	}
}

export const apiFetchSubmissionProgress = async (): Promise<SubmissionProgress> => {
	try {
		const res = await axios.get(API_URL + '/candidate/portfolio/submission_progress', { withCredentials: true });
		return res.data;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to fetch submission progress');
	}
}		

export const apiUploadCoverLetter = async (
	letter: File,
	progressReporter: (progress: AxiosProgressEvent) => void
): Promise<boolean> => {
	try {
		const res = await axios.post(API_URL + '/candidate/add/cover_letter', letter, {
			withCredentials: true,
			data: letter,
			headers: {
				'Content-Type': 'application/pdf',
			},
			onUploadProgress: progressReporter,
		});
		return true;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to upload cover letter');
	}
}

export const apiUploadPortfolioLetter = async (
	letter: File,
	progressReporter: (progress: AxiosProgressEvent) => void
): Promise<boolean> => {
	try {
		const res = await axios.post(API_URL + '/candidate/add/portfolio_letter', letter, {
			withCredentials: true,
			data: letter,
			headers: {
				'Content-Type': 'application/pdf',
			},
			onUploadProgress: progressReporter,
		});
		return true;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to upload cover letter');
	}
}

export const apiUploadPortfolioZip = async (
	portfolio: File,
	progressReporter: (progress: AxiosProgressEvent) => void
): Promise<boolean> => {
	try {
		const res = await axios.post(API_URL + '/candidate/add/portfolio_zip', portfolio, {
			withCredentials: true,
			data: portfolio,
			headers: {
				'Content-Type': 'application/zip',
			},
			onUploadProgress: progressReporter,
		});
		return true;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to upload cover letter');
	}
}
