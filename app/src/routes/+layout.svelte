<script lang="ts">
    import "styles/themes.scss";
    import "styles/global.scss";

    import Nav from "components/layout/Nav.svelte";
    import { syncTheme } from "components/ThemeSelect.svelte";
    import { onMount, type Snippet } from "svelte";
    import { updated } from "$app/state";
    import { version } from "$app/environment";
    import { createSettingsContext } from "lib/settingsContext";
    import SettingsButton from "components/layout/SettingsButton.svelte";
    import { onNavigate } from "$app/navigation";
    import { resolve } from "$app/paths";
    import Settings from "components/Settings.svelte";

    interface Props {
        children?: Snippet;
    }

    const { children }: Props = $props();

    // Setting menu
    const { isOpen, close } = createSettingsContext();
    onNavigate(() => close());

    // Metadata
    const title = "Aracardi - Online Drinking Game";
    const description =
        "Have an unforgettable drinking night with your friends on Aracardi! With over 300 unique cards every round brings fresh and original prompts to enjoy!";

    // Theme
    if (typeof window !== "undefined") syncTheme();

    // SW update state
    let newWorker: ServiceWorker | null;
    let updateReady = $state(false);

    async function prepareUpdate() {
        const reg = await navigator.serviceWorker.getRegistration();
        if (!reg) return;

        // Ensure browser checks for new SW
        await reg.update();

        if (reg.waiting) {
            // Case 1: already waiting
            newWorker = reg.waiting;
            updateReady = true;
            return;
        } else if (reg.installing) {
            // Case 2: installing now
            trackInstalling(reg.installing);
        } else {
            // Case 3: will install soon
            reg.addEventListener("updatefound", () => {
                if (reg.installing) {
                    trackInstalling(reg.installing);
                }
            });
        }
    }

    function trackInstalling(worker: ServiceWorker) {
        worker.addEventListener("statechange", () => {
            if (worker.state === "installed") {
                if (navigator.serviceWorker.controller) {
                    // New version ready
                    newWorker = worker;
                    updateReady = true;
                }
            }
        });
    }

    // Prompt for update
    $effect(() => {
        if (updateReady && newWorker) {
            const confirmed = confirm("A new version is available. Update now?");
            if (confirmed && newWorker) {
                newWorker.postMessage("SKIP_WAITING");
            }
        }
    });

    onMount(async () => {
        const consoleStyle = "background: black;color: gold;";
        console.info("%c##########################", consoleStyle);
        console.info("%c######## Aracardi ########", consoleStyle);
        console.info("%c##### Made by Owlive #####", consoleStyle);
        console.info("%c##########################", consoleStyle);

        await updated.check();

        let versionDate = new Date(Number(version));
        console.info(`Version: ${versionDate.toLocaleDateString()} ${versionDate.toLocaleTimeString()} (${version})`);

        if (updated.current) {
            console.info("Updated detected");

            // Reload if sw changed
            navigator.serviceWorker?.addEventListener("controllerchange", () => {
                window.location.reload();
            });

            prepareUpdate();
        }
    });
</script>

<svelte:head>
    <title>{title}</title>
    <meta name="description" content={description} />
    <meta name="author" content="Owlive" />
    <meta name="theme-color" content="#deae54" />

    <meta property="og:title" content={title} />
    <meta property="og:description" content={description} />
    <meta property="og:image" content="/banner.webp?a" />
    <meta name="twitter:card" content="summary_large_image" />
    <meta property="og:image:width" content="1920" />
    <meta property="og:image:height" content="1080" />
    <meta property="og:image:type" content="image/webp" />
    <meta property="og:locale" content="en_UK" />
    <meta property="og:locale:alternate" content="en_US" />
    <meta property="og:url" content="https://aracardi.com" />
    <meta property="og:type" content="website" />
</svelte:head>

<h1><a href={resolve("/")}>Aracardi</a></h1>
<div class:hidden={$isOpen}>
    {@render children?.()}
</div>

{#if $isOpen}
    <Settings />
{/if}
<SettingsButton />

<Nav />

<style lang="scss">
    div {
        display: contents;
    }

    .hidden {
        display: none;
    }

    h1 {
        font-weight: 200;
        font-size: clamp(2.5rem, 5vw, 5rem);
        text-transform: capitalize;
        padding: 0.2em;
        text-align: center;
    }
</style>
