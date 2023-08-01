<script lang="ts">
    import type { PageData } from "./$types";
    import Game from "components/Game.svelte";

    export let data: PageData;
</script>

<section class="games">
    {#each data.games as game (game.id)}
        <Game {game} />
    {/each}
</section>

<style lang="scss">
    .games {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        gap: 1rem;
    }
</style>
