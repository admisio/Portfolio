import axios, { type AxiosProgressEvent } from 'axios';
import type { CandidateData, CandidateLogin } from '$lib/stores/candidate';
import type { SubmissionProgress } from '$lib/stores/portfolio';
import { API_URL, errorHandler, type Fetch } from '.';
import DOMPurify from 'isomorphic-dompurify';

// SSR Compatible
export const apiLogout = async (fetchSsr?: Fetch) => {
	const apiFetch = fetchSsr || fetch;
	try {
		const res = await apiFetch(API_URL + '/candidate/logout', {
			method: 'POST',
			credentials: 'include'
		});
		return await res.json();
	} catch (e) {
		throw errorHandler(e, 'Logout failed');
	}
};

// SSR Compatible
export const apiFetchDetails = async (fetchSsr?: Fetch): Promise<CandidateData> => {
	const apiFetch = fetchSsr || fetch;
	try {
		const res = await apiFetch(API_URL + '/candidate/details', {
			method: 'GET',
			credentials: 'include'
		});
		if (res.status != 200) {
			throw new Error(await res.text());
		}
		return await res.json();
	} catch (e) {
		throw errorHandler(e, 'Fetch details failed');
	}
};

// SSR Compatible
export const apiFetchSubmissionProgress = async (fetchSsr?: Fetch): Promise<SubmissionProgress> => {
	const apiFetch = fetchSsr || fetch;
	try {
		const res = await apiFetch(API_URL + '/candidate/portfolio/submission_progress', {
			method: 'GET',
			credentials: 'include'
		});
		if (res.status != 200) {
			throw Error(await res.text());
		}
		return await res.json();
	} catch (e) {
		throw errorHandler(e, 'Failed to fetch submission progress');
	}
};

export const apiWhoami = async (fetchSsr?: Fetch): Promise<string> => {
	const apiFetch = fetchSsr || fetch;
	try {
		console.log(API_URL + '/candidate/whoami');
		const res = await apiFetch(API_URL + '/candidate/whoami', {
			method: 'GET',
			credentials: 'include'
		});
		if (res.status != 200) {
			throw Error(await res.text());
		}
		return await res.text();
	} catch (e) {
		throw errorHandler(e, 'Failed to fetch whoami');
	}
};

export const apiLogin = async (data: CandidateLogin): Promise<number> => {
	try {
		const res = await axios.post(API_URL + '/candidate/login', data, { withCredentials: true });
		return data.applicationId;
	} catch (e: any) {
		throw errorHandler(e, 'Login failed');
	}
};

export const apiFillDetails = async (data: CandidateData): Promise<CandidateData> => {
	Object.keys(data.candidate).forEach((key) => {
		// eslint-disable-next-line @typescript-eslint/ban-ts-comment
		// @ts-ignore
		data[key] = DOMPurify.sanitize(data[key]);
	});
	data.parents.forEach((parent) => {
		Object.keys(parent).forEach((key) => {
			// eslint-disable-next-line @typescript-eslint/ban-ts-comment
			// @ts-ignore
			parent[key] = DOMPurify.sanitize(parent[key]);
		});
	});
	console.log(data);
	try {
		const res = await axios.post(API_URL + '/candidate/details', data, { withCredentials: true });
		return res.data;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to fill details');
	}
};

export const apiUploadCoverLetter = async (
	letter: File,
	progressReporter: (progress: AxiosProgressEvent) => void
): Promise<boolean> => {
	try {
		const res = await axios.post(API_URL + '/candidate/add/cover_letter', letter, {
			withCredentials: true,
			data: letter,
			headers: {
				'Content-Type': 'application/pdf'
			},
			onUploadProgress: progressReporter
		});
		return true;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to upload cover letter');
	}
};

export const apiDeleteCoverLetter = async (): Promise<boolean> => {
	try {
		await axios.delete(API_URL + '/candidate/remove/cover_letter', {
			withCredentials: true,
		});
		return true;
	} catch (e) {
		throw errorHandler(e, 'Failed to delete cover letter');
	}
};

export const apiUploadPortfolioLetter = async (
	letter: File,
	progressReporter: (progress: AxiosProgressEvent) => void
): Promise<boolean> => {
	try {
		const res = await axios.post(API_URL + '/candidate/add/portfolio_letter', letter, {
			withCredentials: true,
			data: letter,
			headers: {
				'Content-Type': 'application/pdf'
			},
			onUploadProgress: progressReporter
		});
		return true;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to upload cover letter');
	}
};

export const apiDeletePortfolioLetter = async (): Promise<boolean> => {
	try {
		await axios.delete(API_URL + '/candidate/remove/portfolio_letter', {
			withCredentials: true,
		});
		return true;
	} catch (e) {
		throw errorHandler(e, 'Failed to delete portfolio letter');
	}
};

export const apiUploadPortfolioZip = async (
	portfolio: File,
	progressReporter: (progress: AxiosProgressEvent) => void
): Promise<boolean> => {
	try {
		const res = await axios.post(API_URL + '/candidate/add/portfolio_zip', portfolio, {
			withCredentials: true,
			data: portfolio,
			headers: {
				'Content-Type': 'application/zip'
			},
			onUploadProgress: progressReporter
		});
		return true;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to upload cover letter');
	}
};

export const apiDeletePortfolioZip = async (): Promise<boolean> => {
	try {
		await axios.delete(API_URL + '/candidate/remove/portfolio_zip', {
			withCredentials: true,
		});
		return true;
	} catch (e) {
		throw errorHandler(e, 'Failed to delete portfolio zip');
	}
};

export const apiSubmitPortfolio = async (): Promise<boolean> => {
	try {
		await axios.post(API_URL + '/candidate/portfolio/submit', {}, { withCredentials: true });
		return true;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to submit portfolio');
	}
};

export const apiDeltePortfolio = async (): Promise<boolean> => {
	try {
		await axios.post(API_URL + '/candidate/portfolio/delete', {}, { withCredentials: true });
		return true;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to delete portfolio');
	}
};
