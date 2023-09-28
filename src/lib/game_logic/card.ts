import type { Stage } from "$lib/lobby/stage";
import type { Card as PrismaCard } from "@prisma/client";

export type Card = PrismaCard & {
    stages: Stage[];
}