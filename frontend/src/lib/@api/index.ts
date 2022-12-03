import type { AxiosError } from "axios";

export type Fetch = (input: RequestInfo, init?: RequestInit) => Promise<Response>

export const API_URL = "http://localhost:9000";

export interface ApiError {
	error: AxiosError,
	msg: string,
}

export function errorHandler(error: AxiosError, msg: string): ApiError {
	return {error, msg}
}