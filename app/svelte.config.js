import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
// import * as child_process from 'node:child_process';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),

    kit: {
        // version: {
        //     name: child_process.execSync('git rev-parse HEAD').toString().trim()
        // },
        serviceWorker: {
            files: (file) => {
                if (file.startsWith("cards/")) {
                    return false;
                }
                if (file.startsWith("branding/")) {
                    return false;
                }

                return true;
            },
        },
        adapter: adapter({
            pages: 'build',
            assets: 'build',
            fallback: "404.html",
            precompress: true,
            strict: true
        }),
        alias: {
            "lib": "./src/lib",
            "styles": "./src/styles",
            "routes": "./src/routes",
            "components": "./src/components",
            "assets": "./src/assets",
            "static": "./static",
        }
    }
};

export default config;
