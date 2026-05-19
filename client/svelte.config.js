import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { execSync } from "child_process";
import pkg from "./package.json" with { type: "json" };

const EXCLUDED_SW_FILES = /^(cards|branding|service-worker|version)\//;

const hash = execSync("git rev-parse --short HEAD").toString().trim();
const dirty = execSync("git status --porcelain").toString().trim() !== "";
const dirtyHash = dirty ? execSync("git diff HEAD | md5sum").toString().trim().slice(0, 8) : "";
const build = dirty ? `${hash}.${dirtyHash}` : hash;

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),

    kit: {
        version: {
            name: `${pkg.version}+${build}`,
            pollInterval: 1_000 * 60 * 10,
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
