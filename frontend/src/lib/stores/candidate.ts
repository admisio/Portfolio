import { writable } from 'svelte/store';

export interface CandidateData {
	candidate: {
		name: string;
		surname: string;
		birthplace: string;
		birthdate: string;
		address: string;
		telephone: string;
		citizenship: string;
		email: string;
		sex: string;
		study: string;
		personalIdNumber: string;
	};
	parents: Array<{
		name: string;
		surname: string;
		telephone: string;
		email: string;
	}>;
}

export interface CandidatePreview {
	applicationId?: number;
	name?: string;
	surname?: string;
	study?: string;
}

export interface CandidateLogin {
	applicationId: number;
	password: string;
}

export interface CreateCandidate {
	applicationId: number;
	personalIdNumber: string;
}

export interface CreateCandidateLogin extends CreateCandidate {
	password: string;
}

export const candidateData = writable<CandidateData>({
	candidate: {
		name: '',
		surname: '',
		birthplace: '',
		birthdate: '',
		address: '',
		telephone: '',
		citizenship: '',
		email: '',
		sex: '',
		study: '',
		personalIdNumber: ''
	},
	parents: []
});
