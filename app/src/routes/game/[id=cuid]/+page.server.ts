import prisma from "$lib/db";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load = (async ({ params }) => {
    const game = await prisma.game.findUnique({
        where: { id: params.id, },
        include: {
            addons: true,
        }
    });

    if (!game) throw error(404, "Game not found");

    return { game };
}) satisfies PageServerLoad;