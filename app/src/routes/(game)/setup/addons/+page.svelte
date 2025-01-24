<script lang="ts">
    import Addon from "components/Addon.svelte";
    import AnchorButton from "components/input/AnchorButton.svelte";

    let { data } = $props();
</script>

<main>
    <div class="addons">
        {#each data.addons as addon (addon.id)}
            <Addon {addon} />
        {/each}
    </div>
    <div>
        <AnchorButton href="/setup/players">Continue</AnchorButton>
    </div>
</main>

<style lang="scss">
    main {
        margin-inline: auto;
        width: min(100%, 80ch);
    }

    .addons {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }
</style>
