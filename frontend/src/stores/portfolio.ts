import { apiFetchSubmissionProgress } from "../@api/candidate";
import { writable } from "svelte/store";

export type Status = 'submitted' | 'uploaded' | 'missing';

export enum UploadStatus {
    None = 1,
    Some = 2,
    All = 3,
    Submitted = 4,
}

export interface SubmissionProgress {
    status?: UploadStatus;
    files?: [number];
}
export const submissionProgress = writable<SubmissionProgress>({});

export async function fetchSubmProgress() {
    try {
        let prog = await apiFetchSubmissionProgress();
        submissionProgress.set(prog);
    } catch (e) {
        console.error(e);
    }
}