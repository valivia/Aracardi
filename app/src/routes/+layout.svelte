<script lang="ts">
    import "styles/themes.scss";
    import "styles/global.scss";

    import Nav from "components/layout/Nav.svelte";
    import { syncTheme } from "components/theme";
    import { onMount, type Snippet } from "svelte";
    import { updated } from "$app/state";
    import { version } from "$app/environment";
    import { createSettingsContext } from "lib/settingsContext";
    import SettingsButton from "components/layout/SettingsButton.svelte";
    import { afterNavigate, onNavigate } from "$app/navigation";
    import { resolve } from "$app/paths";
    import Settings from "components/Settings.svelte";

    interface Props {
        children?: Snippet;
    }

    const { children }: Props = $props();

    // Setting menu
    const { isOpen, close } = createSettingsContext();

    // Close settings menu when navigating to different page.
    onNavigate(() => close());

    // Metadata
    const title = "Aracardi - Online Drinking Game";
    const description =
        "Have an unforgettable drinking night with your friends on Aracardi! With over 300 unique cards every round brings fresh and original prompts to enjoy!";

    // Theme
    if (typeof window !== "undefined") syncTheme();

    let swRegistration: ServiceWorkerRegistration | undefined;

    onMount(async () => {
        const consoleStyle = "background: black;color: gold;";
        console.info("%c##########################", consoleStyle);
        console.info("%c######## Aracardi ########", consoleStyle);
        console.info("%c##### Made by Owlive #####", consoleStyle);
        console.info("%c##########################", consoleStyle);

        let [semver, build] = version.split("+");
        console.info(`Version: ${semver} (${build})`);

        await updated.check();
        swRegistration = await navigator.serviceWorker?.getRegistration();
    });

    async function checkUpdate() {
        if (!swRegistration) return;
        console.log("[sw] Fetching update");
        swRegistration = await swRegistration.update();
    }

    $effect(() => {
        if (updated.current) checkUpdate();
    });

    afterNavigate(async () => {
        if (!swRegistration?.waiting) return;
        console.log("[sw] Forcing activation");

        const waiting = swRegistration.waiting;

        function onStateChange(this: ServiceWorker) {
            if (this.state === "activated") {
                console.log("[sw] Forcing reload");
                waiting.removeEventListener("statechange", onStateChange);
                window.location.reload();
            }
        }

        waiting.addEventListener("statechange", onStateChange);
        waiting.postMessage("SKIP_WAITING");
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
