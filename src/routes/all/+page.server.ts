import { readdir } from "fs/promises";
import type { PageServerLoad } from './$types';
import type { Addon } from "lib/addon";

export const load = (async () => {
    const files = await readdir("./src/assets/addons", { withFileTypes: true });

    const addons = await Promise.all(
        files
            .filter((file) => file.isFile() && file.name.endsWith(".json"))
            .map(async (file) => {
                const fileName = file.name.replace(".json", "");
                const json = await import(`assets/addons/${fileName}.json`)
                return json.default as Addon;
            })
        );

return { addons };
}) satisfies PageServerLoad;
