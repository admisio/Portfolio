import { apiFetchDetails, apiFillDetails, apiLogin, apiLogout } from "../@api/candidate";
import { writable } from "svelte/store";
import { browser } from "$app/environment";

export interface CandidateData {
    name?: string;
    surname?: string;
    birthplace?: string;
    birthdate?: string;
    address?: string;
    telephone?: string;
    citizenship?: string;
    email?: string;
    sex?: string;
    study?: string;
    personalIdNumber?: string;
    parentName?: string;
    parentSurname?: string;
    parentTelephone?: string;
    parentEmail?: string;
}

export interface CandidateLogin {
    applicationId: number;
    password: string;
}

export const candidateData = writable<CandidateData>({});


if (browser) {
    const name = localStorage.getItem("name");
    const surname = localStorage.getItem("surname");
    const email = localStorage.getItem("email");
    if (name && email && surname) {
        candidateData.set({
            name,
            surname,
            email
        });
    }
}
candidateData.subscribe((val) => {
    if (browser) {
        localStorage.setItem("name", val.name ?? "");
        localStorage.setItem("surname", val.surname ?? "");
        localStorage.setItem("email", val.email ?? "");
    }
})


export async function login(data: CandidateLogin) {
    // TODO: handle errors
    try {
        const applicationId = await apiLogin(data); // TODO: set candidate data from response to store
        console.log("login result: " + applicationId);
    } catch (e) {
        console.error("login failed");
        throw e;
    }
}

export async function logout() {
    // TOOD: handle errors
    try {
        await apiLogout();
        candidateData.set({});
    } catch (e) {
        console.error(e);
    }
}

export async function fillDetails(data: CandidateData) {
    try {
        const res = await apiFillDetails(data);
        candidateData.set(res);
    } catch (e) {
        console.error(e);
        throw e;
    }
}

export async function fetchDetails() {
    try {
        const res = await apiFetchDetails();
        candidateData.set(res);
    } catch (e) {
        console.error(e);
        throw e;
    }
}