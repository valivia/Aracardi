import type { Addon, AddonSummary } from "lib/addon";
import type { PageServerLoad } from "../$types";
import { readdir } from "fs/promises";

export const load = (async () => {
    const addonFiles = await readdir("static/addons", { withFileTypes: true });

    const addons: AddonSummary[] = await Promise.all(addonFiles
        .filter(file => file.isFile() && file.name.endsWith(".json"))
        .map(async file => {
            const fileName = file.name.replace(".json", "");
            const json = await import(`static/addons/${fileName}.json`)
            const importedAddon = json.default as Addon;
            const addon: AddonSummary = {
                id: importedAddon.id,
                title: importedAddon.title,
                description: importedAddon.description,
                isOfficial: importedAddon.isOfficial,
                cardCount: importedAddon.cards.length,
                nsfwCardCount: importedAddon.cards.filter(card => card.isNsfw).length
            };

            return addon;
        }));

    return { addons };
}) satisfies PageServerLoad;
