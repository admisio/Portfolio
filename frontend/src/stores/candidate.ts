import { apiLogin, apiLogout } from "../api/candidate";
import { writable } from "svelte/store";

export interface CandidateData {
    id?: string;
    name?: string;
    surname?: string;
    email?: string;
}

export interface CandidateLogin {
    application_id: number;
    password: string;
}

export const candidateData = writable<CandidateData>();

export async function login(data: CandidateLogin) {
    // TODO: handle errors
    let res = await apiLogin(data); // TODO: set candidate data from response to store
}

export async function logout() {
    // TOOD: handle errors
    await apiLogout();
    candidateData.set({});
}