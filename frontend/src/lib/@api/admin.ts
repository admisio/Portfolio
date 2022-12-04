import type { AdminLogin } from "$lib/stores/admin";
import type { CandidateData, CandidatePreview, CreateCandidate, CreateCandidateLogin } from "$lib/stores/candidate";
import axios from "axios";
import { API_URL, errorHandler, type Fetch } from ".";


// Login as admin /admin/login
export const apiLogin = async (data: AdminLogin): Promise<number> => {
	try {
		const res = await axios.post(API_URL + '/admin/login', data, { withCredentials: true });
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
}

// Reset candidate password /admin/candidate/{id}/reset_password
export const apiResetCandidatePassword = async (id: number): Promise<CreateCandidateLogin> => {
    try {
        const res = await axios.post(API_URL + '/admin/candidate/' + id + '/reset_password',
            {},
            { withCredentials: true }
        );
        return res.data;
    } catch (e: any) {
        throw errorHandler(e, 'Candidate creation failed');
    }
}

export const apiGetCandidatePortfolio = async (id: number): Promise<Blob> => {
	try {
		const res = await fetch(API_URL + '/admin/candidate/' + id + '/portfolio', {
			method: 'GET',
			credentials: 'include',
		});
		return await res.blob();
	} catch (e: any) {
		throw errorHandler(e, 'Candidate portfolio failed');
	}
}

// SSR compatible
// Logout as admin /admin/logout
export const apiLogout = async (fetchSsr?: Fetch) => {
	try {
		if (fetchSsr) {
			const res = await fetchSsr(API_URL + '/admin/logout', {
				method: 'POST',
				credentials: 'include'
			});
			return await res.text();
		}
		const res = await axios.post(API_URL + '/admin/logout', { withCredentials: true });
		return res.data;
	} catch (e: any) {
		throw errorHandler(e, 'Logout failed');
	}
};

// SSR compatible
// List all candidates /admin/list/candidates
export const apiListCandidates = async (fetchSsr?: Fetch): Promise<[CandidatePreview]> => {
	try {
		if (fetchSsr) {
			const res = await fetchSsr(API_URL + '/admin/list/candidates', {
				method: 'GET',
				credentials: 'include'
			});
			return await res.json();
		}
		const res = await axios.get(API_URL + '/admin/list/candidates', {
			withCredentials: true
		});
		return res.data;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to fetch submission progress');
	}
};

// SSR compatible
// Get candidate data /admin/candidate/{id}
export const apiFetchCandidate = async (id: number, fetchSsr?: Fetch): Promise<CandidateData> => {
    try {
		if (fetchSsr) {
			const res = await fetchSsr(API_URL + '/admin/candidate/' + id, {
				method: 'GET',
				credentials: 'include'
			});
			return await res.json();
		}
		const res = await axios.get(API_URL + '/admin/candidate/' + id, {
			withCredentials: true
		});
		return res.data;
	} catch (e: any) {
		throw errorHandler(e, 'Failed to fetch candidate data');
	}
}