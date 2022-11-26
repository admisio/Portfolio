export const API_URL = "http://localhost:8000";

export function errorHandler(error: { data?: { [key: string]: string[] } }, fallbackMessage: string) {
	return error?.data?.errors ? error.data.errors : { unknown: [fallbackMessage] };
}