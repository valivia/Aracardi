// Disables access to DOM typings like `HTMLElement` which are not available
// inside a service worker and instantiates the correct globals
/// <reference no-default-lib="true"/>
/// <reference lib="esnext" />
/// <reference lib="webworker" />

// Ensures that the `$service-worker` import has proper type definitions
/// <reference types="@sveltejs/kit" />

// Only necessary if you have an import from `$env/static/public`
/// <reference types="../.svelte-kit/ambient.d.ts" />

import { build, files, version } from "$service-worker";

// This gives `self` the correct types
const self = globalThis.self as unknown as ServiceWorkerGlobalScope;

// Create a unique cache name for this deployment
const CACHE = `cache-${version}`;

const ASSETS = [
    ...build, // the app itself
    ...files, // everything in `static`
    "/",
    "/game",
];

self.addEventListener("install", (event) => {
    console.log(`[sw] Installing with ${ASSETS.length} files`, version);

    async function addFilesToCache() {
        const cache = await caches.open(CACHE);
        await cache.addAll(ASSETS);
    }

    const waitUntil = addFilesToCache().then(() => console.log(`[sw] finished installing`, version));

    event.waitUntil(waitUntil);
});

self.addEventListener("activate", (event) => {
    console.log(`[sw] Activating with ${ASSETS.length}`, version);
    console.log({ assets: ASSETS });
    // Remove previous cached data from disk
    async function deleteOldCaches() {
        for (const key of await caches.keys()) {
            if (key !== CACHE) await caches.delete(key);
        }
    }

    const waitUntil = Promise.all([deleteOldCaches(), self.clients.claim()]).then(() =>
        console.log(`[sw] finished activating`, version),
    );

    event.waitUntil(waitUntil);
});

async function handleRequest(request: Request, url: URL) {
    const cache = await caches.open(CACHE);

    // `build`/`files` can always be served from the cache
    if (ASSETS.includes(url.pathname)) {
        const response = await cache.match(url.pathname);

        if (response) {
            return response;
        }
    }

    // for everything else, try the network first, but
    // fall back to the cache if we're offline
    try {
        const response = await fetch(request);

        // if we're offline, fetch can return a value that is not a Response
        // instead of throwing - and we can't pass this non-Response to respondWith
        if (!(response instanceof Response)) {
            throw new Error("invalid response from fetch");
        }

        if (response.status === 200 && !response.headers.get("cache-control")?.includes("no-store")) {
            cache.put(request, response.clone());
        }

        return response;
    } catch (err) {
        const response = await cache.match(request);

        if (response) {
            return response;
        }

        // if there's no cache, then just error out
        // as there is nothing we can do to respond to this request
        throw err;
    }
}

self.addEventListener("fetch", (event) => {
    // ignore POST requests etc
    if (event.request.method !== "GET") return;

    const url = new URL(event.request.url);
    // Only urls on own domain
    if (url.hostname !== self.location.hostname) return;
    // Ignore server requests
    if (url.pathname.startsWith("/server")) return;
    // Always fetch version over network
    if (url.pathname === "/_app/version.json") return;
    // ignore card images to save space
    if (url.pathname.startsWith("/cards/")) return;

    event.respondWith(handleRequest(event.request, url));
});

self.addEventListener("message", (event) => {
    if (event.data === "SKIP_WAITING") {
        console.log(`[sw] Received SKIP_WAITING`, version);
        self.skipWaiting();
    }
});
