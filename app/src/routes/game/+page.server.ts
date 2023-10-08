import prisma from "$lib/db";
import type { PageServerLoad } from "./$types";

export const load = (async () => {
    const games = await prisma.game.findMany();
    return { games };
}) satisfies PageServerLoad;