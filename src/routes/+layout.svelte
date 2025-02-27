<script lang="ts">
    import Nav from "components/layout/Nav.svelte";
    import type { Snippet } from "svelte";

    interface Props {
        children?: Snippet;
    }

    const { children }: Props = $props();

    const description = `
        Have an unforgettable drinking night with your friends on Aracardi!
        Flip the card to reveal fresh, fun, and original prompts every time.
        With over 300 unique cards across several card packs categories, every round is packed with excitement!`;
</script>

<svelte:head>
    <title>Aracardi</title>
    <meta name="description" content={description} />
    <meta name="author" content="Owlive" />
    <meta name="theme-color" content="#deae54" />

    <meta property="og:title" content="Aracardi - Online Drinking Game" />
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
    @use "styles/global.scss" as *;

    h1 {
        padding: 0.2em;
        text-align: center;
    }
</style>
