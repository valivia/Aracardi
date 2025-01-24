import type { AddonSummary } from "lib/addon";
import type { Player } from "lib/lobby/player";
import { SvelteMap } from "svelte/reactivity";

export const selectedAddons: Map<string, AddonSummary> = $state(new SvelteMap());

export const players: Player[] = $state([]);
