import type { Addon, AddonSummary } from "lib/addon";
import { readdir } from "fs/promises";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
    const addonFiles = await readdir("./src/assets/addons", { withFileTypes: true });

    const addons: AddonSummary[] = await Promise.all(addonFiles
        .filter(file => file.isFile() && file.name.endsWith(".json"))
        .map(async (file) => {
            const fileName = file.name.replace(".json", "");
            const { default: importedAddon } = await import(`assets/addons/${fileName}.json`);

            const addon = importedAddon as Addon;

            const addonSummary: AddonSummary = {
                fileName,
                id: addon.id,
                title: addon.title,
                description: addon.description,
                isDefault: addon.isDefault,
                isOfficial: addon.isOfficial,
                cardCount: addon.cards.length,
                nsfwCardCount: addon.cards.filter(card => card.isNsfw).length,
            };

            return addonSummary;
        })
    );

    addons.sort((a, b) => {
        if (b.isDefault !== a.isDefault) {
            return b.isDefault ? 1 : -1;
        }

        if (b.isOfficial !== a.isOfficial) {
            return b.isOfficial ? 1 : -1;
        }

        return b.cardCount - a.cardCount;
    });

    return { addons };
};
