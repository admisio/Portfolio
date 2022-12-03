import type { ParamMatcher } from '@sveltejs/kit';

export const match: ParamMatcher = (param) => {
	const isNumber = /^\d{6}(?:\d{1})?$/.test(param);
	const isValid = param.startsWith('101') || param.startsWith('102') || param.startsWith('103');
	
	return isNumber && isValid;
};
