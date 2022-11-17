import type { ParamMatcher } from '@sveltejs/kit';

export const match: ParamMatcher = (param) => {
	//TODO: Application regex
	return /^\d+$/.test(param);
};
