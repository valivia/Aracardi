import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

const EXCLUDED_SW_FILES = /^(cards|branding|service-worker|version)\//;

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),

    kit: {
        version: {
            name: String(Date.now()),
        },
        serviceWorker: {
            files: (file) => !EXCLUDED_SW_FILES.test(file),
        },
        adapter: adapter({
            pages: "build",
            assets: "build",
            fallback: "404.html",
            precompress: true,
            strict: true,
        }),
        alias: {
            lib: "./src/lib",
            styles: "./src/styles",
            routes: "./src/routes",
            components: "./src/components",
            assets: "./src/assets",
            static: "./static",
        },
    },
};

export default config;
