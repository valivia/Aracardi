import type { PageLoad } from './$types';

export const load: PageLoad = ({ url }) => {
    const lobby = url.searchParams.get("lobby");
    if (!lobby) {
        throw new Error("Lobby ID is required");
    }
    return { lobby };
};
