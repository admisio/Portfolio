import { goto } from "$app/navigation";
import type { AxiosError } from "axios";

export const API_URL = "http://localhost:9000";

export interface ApiError {
	error: AxiosError,
	msg: string,
}

export function errorHandler(error: AxiosError, msg: string): ApiError {
	return {error, msg}
}