import { apiFetchSubmissionProgress } from "../@api/candidate";
import { writable } from "svelte/store";

export interface SubmissionProgress {
    status?: number;
    files?: [number];
}

export const submissionProgress = writable<SubmissionProgress>();

export async function fetchSubmProgress() {
    try {
        let prog = await apiFetchSubmissionProgress();
        submissionProgress.set(prog);
    } catch (e) {
        console.error(e);
    }
}