<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<ul>
    {#each data.game.addons as addon (addon.id)}
        <li>{addon.title}</li>
    {/each}
</ul>

<style lang="scss">
</style>
