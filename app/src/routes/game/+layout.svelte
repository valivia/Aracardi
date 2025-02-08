<script lang="ts">
    const { children } = $props();
</script>

<svelte:head>
    <title>Aracardi - setup</title>
</svelte:head>

{@render children?.()}
