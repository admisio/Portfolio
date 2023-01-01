import type { AdminLogin } from '$lib/stores/admin';
import type {
	CandidateData,
	CandidatePreview,
	CreateCandidate,
	CreateCandidateLogin
} from '$lib/stores/candidate';
import axios from 'axios';
import { API_URL, errorHandler, type Fetch } from '.';

// Login as admin /admin/login
export const apiLogin = async (data: AdminLogin): Promise<number> => {
	try {
		await axios.post(API_URL + '/admin/login', data, { withCredentials: true });
		return data.adminId;
	} catch (e: any) {
		throw errorHandler(e, 'Login failed');
	}
};

// Create new candidate /admin/create
// return created candidate's applicationId, personalIdNumber and password
export const apiCreateCandidate = async (data: CreateCandidate): Promise<CreateCandidateLogin> => {
	try {
		const res = await axios.post(API_URL + '/admin/create', data, { withCredentials: true });
		return res.data;
	} catch (e: any) {
		throw errorHandler(e, 'Candidate creation failed');
	}
};

// Deletes candidate /admin/candidate/{id}
export const apiDeleteCandidate = async (id: number): Promise<string> => {
	try {
		const res = await axios.delete(API_URL + `/admin/candidate/${id}`, { withCredentials: true });
		return res.data;
	} catch (e) {
		throw errorHandler(e, 'Candidate creation failed');
	}
};

// Reset candidate password /admin/candidate/{id}/reset_password
export const apiResetCandidatePassword = async (id: number): Promise<CreateCandidateLogin> => {
	try {
		const res = await axios.post(
			API_URL + '/admin/candidate/' + id + '/reset_password',
			{},
			{ withCredentials: true }
		);
		return res.data;
	} catch (e: any) {
		throw errorHandler(e, 'Candidate creation failed');
	}
};

export const apiGetCandidatePortfolio = async (id: number): Promise<Blob> => {
	try {
		const res = await fetch(API_URL + '/admin/candidate/' + id + '/portfolio', {
			method: 'GET',
			credentials: 'include'
		});
		return await res.blob();
	} catch (e: any) {
		throw errorHandler(e, 'Candidate portfolio failed');
	}
};

// SSR compatible
// Logout as admin /admin/logout
export const apiLogout = async (fetchSsr?: Fetch) => {
	const apiFetch = fetchSsr || fetch;

	try {
		const res = await apiFetch(API_URL + '/admin/logout', {
			method: 'POST',
			credentials: 'include'
		});
		return await res.text();
	} catch (e) {
		throw errorHandler(e, 'Logout failed');
	}
};

// SSR compatible
// List all candidates /admin/list/candidates
export const apiListCandidates = async (
	fetchSsr?: Fetch,
	field?: string
): Promise<Array<CandidatePreview>> => {
	const apiFetch = fetchSsr || fetch;
	const params = new URLSearchParams();
	if (field) {
		params.append('field', field);
	}
	try {
		const res = await apiFetch(API_URL + '/admin/list/candidates?' + params.toString(), {
			method: 'GET',
			credentials: 'include'
		});
		if (res.status != 200) {
			throw Error(await res.text());
		}
		return await res.json();
	} catch (e) {
		throw errorHandler(e, 'List candidates failed');
	}
};

// SSR compatible
// Get candidate data /admin/candidate/{id}
export const apiFetchCandidate = async (id: number, fetchSsr?: Fetch): Promise<CandidateData> => {
	const apiFetch = fetchSsr || fetch;
	try {
		const res = await apiFetch(API_URL + '/admin/candidate/' + id, {
			method: 'GET',
			credentials: 'include'
		});
		return await res.json();
	} catch (e) {
		throw errorHandler(e, 'Failed to fetch candidate data');
	}
};

// SSR compatible
// List all candidates /admin/list/candidates
export const apiListCandidatesCSV = async (
	fetchSsr?: Fetch,
): Promise<Blob> => {
	const apiFetch = fetchSsr || fetch;
	try {
		const res = await apiFetch(API_URL + '/admin/list/candidates_csv', {
			method: 'GET',
			credentials: 'include'
		});
		if (res.status != 200) {
			throw Error(await res.text());
		}
		return await res.blob();
	} catch (e) {
		throw errorHandler(e, 'List candidates CSV failed');
	}
};
