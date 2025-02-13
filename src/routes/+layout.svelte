<script lang="ts">
    import Nav from "components/layout/Nav.svelte";
    import type { Snippet } from "svelte";

    interface Props {
        children?: Snippet;
    }

    const { children }: Props = $props();
</script>

<svelte:head>
    <title>Aracardi</title>
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
