import type { ParamMatcher } from "@sveltejs/kit";

export const match = ((param) => {
    return param.length === 25 && param.startsWith("c");
}) satisfies ParamMatcher;