import { Dirent, readdirSync } from "fs";
import path from "path";
import { BASE_PATH } from ".";
import { readFile, unlink } from "fs/promises";
import type { PrototypeAddon } from "lib/addon";
import type { PrototypeCard } from "lib/card.svelte";
import sharp from "sharp";

export class ImageService {
    private images: Dirent[];
    private cards: PrototypeCard[];

    constructor(addons: PrototypeAddon[]) {
        this.cards = addons.flatMap(addon => addon.cards);
        this.images = readdirSync(path.join(BASE_PATH, "cards"), { withFileTypes: true });
    }

    public findRedundantImages(): Dirent[] {
        const cardIds = new Set<string>();

        for (const card of this.cards) {
            if (card.id) cardIds.add(card.id);
        }

        return this.images.filter(file => file.isFile() && file.name.endsWith(".webp") && !cardIds.has(file.name.slice(0, -5)));
    }

    public findCardsWithMissingImage(): PrototypeCard[] {
        const cardsWithImage = this.cards.filter(card => card.image);

        const images = this.images.map(image => image.name.slice(0, -5));

        return cardsWithImage.filter(card => {
            if (!card.id) console.error(`Card "${card.id}" has no id`);
            return card.id && !images.includes(card.id);
        });
    }

    public static async deleteImage(id: string | Dirent): Promise<void> {
        const fileName = typeof id === "string" ? `${id}.webp` : id.name;
        const filePath = path.join(BASE_PATH, "cards", fileName);

        try {
            await unlink(filePath);
            console.log(`🧹 Image ${fileName} deleted successfully.`);
        } catch (error) {
            console.error(`❌ Error deleting image ${fileName}:`, error);
            throw error;
        }
    }

    public static async saveImage(id: string, fileName: string): Promise<void> {
        try {
            const sourcePath = path.join("import", fileName);
            const outputPath = path.join(BASE_PATH, "cards", `${id}.webp`);

            // Read file safely
            let fileBuffer: Buffer;
            try {
                fileBuffer = await readFile(sourcePath);
            } catch {
                throw new Error(`Image not found: ${fileName}`);
            }

            // Get image metadata
            const metadata = await sharp(fileBuffer).metadata();
            if (!metadata.width || !metadata.height) {
                throw new Error(`Invalid image dimensions: ${fileName}`);
            }

            const aspectRatio = 16 / 9;
            let resizeOptions: sharp.ResizeOptions;

            if (metadata.width > 720 && metadata.height > 480) {
                // Downscale to 16:9
                resizeOptions = { width: 720, height: 480, fit: "cover" };
            } else {
                // Crop to 16:9 while keeping original resolution
                const newWidth = metadata.width;
                const newHeight = Math.round(metadata.width / aspectRatio);

                if (newHeight > metadata.height) {
                    // Adjust width instead if height exceeds original
                    resizeOptions = { width: Math.round(metadata.height * aspectRatio), height: metadata.height, fit: "cover" };
                } else {
                    resizeOptions = { width: newWidth, height: newHeight, fit: "cover" };
                }
            }

            const output = await sharp(fileBuffer, { pages: -1 })
                .rotate()
                .resize(resizeOptions)
                .webp({ quality: 80 })
                .toFile(outputPath);

            console.log(`💾 Image for card ${id} saved successfully. (${this.formatBytes(output.size)})`);
        } catch (error) {
            console.error(`❌ Error saving image for card ${id}:`, error);
            throw error;
        }
    }

    private static formatBytes(bytes: number, decimals = 2) {
        if (!+bytes) return '0 Bytes'

        const k = 1024
        const dm = decimals < 0 ? 0 : decimals
        const sizes = ['Bytes', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB']

        const i = Math.floor(Math.log(bytes) / Math.log(k))

        return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`
    }
}
