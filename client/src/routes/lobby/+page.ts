import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = ({ url }) => {
    const joinCode = url.searchParams.get("join_code");
    if (!joinCode || joinCode.length !== 6) {
        error(400, "Invalid join code");
    }
    return { lobby: joinCode };
};
