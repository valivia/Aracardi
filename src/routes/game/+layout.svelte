<script lang="ts">
    const { children } = $props();
</script>

<svelte:head>
    <title>Aracardi - setup</title>
    <meta name="robots" content="noindex" />
</svelte:head>

{@render children?.()}
