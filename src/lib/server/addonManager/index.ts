import path from "path";
import { AddonManager } from "./addon";
import { ImageService } from "./image";

export const BASE_PATH = path.resolve("./src/assets");
export const ASSET_PATH = path.resolve("./static/cards");


async function main(destructive = false) {

    if (process.env.DESTRUCTIVE === "true") {
        destructive = true;
    }

    console.log(`🔃 Checking addons${destructive ? " (destructive)" : ""}`);

    let addons = await AddonManager.getAddons();

    let hasErrors = false;

    try {
        for (const [name, addon] of Object.entries(addons)) {
            const transformed = await new AddonManager(addon, name).check(destructive);

            if (transformed === null) {
                console.log(`❌ Addon "${name}" has errors`);
                hasErrors = true;
                continue;
            }

            if (destructive) await transformed.save();

            console.log("==============================");
        }

        console.log("✅ All addons validated successfully.");
    } catch (error) {
        console.error("❌ Error processing addons:", error);
        throw error;
    }

    if (hasErrors) {
        throw "❌ Some addons have errors";
    }

    if (destructive)
        addons = await AddonManager.getAddons();


    const imageController = new ImageService(Object.values(addons));

    // Check for redundant images
    const redundantImages = imageController.findRedundantImages();
    if (redundantImages.length) {
        console.log(`🔃 Found ${redundantImages.length} dangling images:`);
        for (const image of redundantImages) {
            if (destructive) ImageService.deleteImage(image);
            else console.log(`  - ${image.name}`);
        }
        if (destructive) console.log("✅ Removed all dangling images");
        else throw new Error("Dangling images found");
    }

    // Check for missing images
    const missingImages = imageController.findCardsWithMissingImage();
    if (missingImages.length) {
        console.log(`🔃 Found ${missingImages.length} cards with missing images:`);
        for (const card of missingImages) {
            console.log(`  - ${card.id || card.title}`);
        }
        throw new Error("Missing images found");
    }
}

main().catch((error) => {
    console.error(error);
    process.exit(1);
});
