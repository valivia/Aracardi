<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<ul>
    {#each data.game.addons as addon (addon.id)}
        <li>
            <a href={`game/${addon.id}`}>{addon.title}</a>
        </li>
    {/each}
</ul>

<style lang="scss">
</style>
