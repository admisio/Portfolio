import type { GradeBackend } from '$lib/components/grades/GradesTable.svelte';
import { writable } from 'svelte/store';


export interface School {
	name: string;
	field: string;
}
export interface CandidateData {
	candidate: {
		name: string;
		surname: string;
		birthSurname: string;
		birthplace: string;
		birthdate: string;
		address: string;
		letterAddress: string;
		telephone: string;
		citizenship: string;
		email: string;
		sex: string;
		personalIdNumber: string;
		schoolName: string;
		healthInsurance: string;
		grades: Array<GradeBackend>;
		firstSchool: School;
		secondSchool: School;
		testLanguage: string;
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
	fieldOfStudy?: string;
}

export interface CandidateLogin {
	applicationId: number;
	password: string;
}

export interface CreateCandidate {
	applicationId: number;
	personalIdNumber: string;
}

export interface BaseCandidate {
	currentApplication: number;
	applications: Array<number>;
	personalIdNumber: string;
	detailsFilled: boolean;
	encryptedBy?: number;
}

export interface CreateCandidateLogin extends CreateCandidate {
	password: string;
}

export const baseCandidateData = writable<BaseCandidate>({
	currentApplication: 0,
	applications: [],
	personalIdNumber: '',
	detailsFilled: false
});

export const candidateData = writable<CandidateData>({
	candidate: {
		name: '',
		surname: '',
		birthSurname: '',
		birthplace: '',
		birthdate: '',
		address: '',
		letterAddress: '',
		telephone: '',
		citizenship: '',
		email: '',
		sex: '',
		personalIdNumber: '',
		schoolName: '',
		healthInsurance: '',
		grades: [],
		firstSchool: {name: '', field: ''},
		secondSchool: {name: '', field: ''},
		testLanguage: ''
	},
	parents: []
});
