<script lang="ts">
    import Nav from "components/layout/Nav.svelte";
    import { syncTheme } from "components/ThemeSelect.svelte";
    import { type Snippet } from "svelte";
    import "styles/themes.scss";
    import "styles/global.scss";

    interface Props {
        children?: Snippet;
    }

    const { children }: Props = $props();

    const title = "Aracardi - Online Drinking Game";
    const description =
        "Have an unforgettable drinking night with your friends on Aracardi! With over 300 unique cards every round brings fresh and original prompts to enjoy!";

    if (typeof window !== "undefined") syncTheme();
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

<h1>Aracardi</h1>
{@render children?.()}
<Nav />

<style lang="scss">
    h1 {
        padding: 0.2em;
        text-align: center;
    }
</style>
