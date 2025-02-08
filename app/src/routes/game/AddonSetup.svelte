<script lang="ts">
    import Addon from "components/Addon.svelte";
    import Button from "components/input/Button.svelte";
    import type { AddonSummary } from "lib/addon";
    import type { GameState } from "./state.svelte";

    interface Props {
        addons: AddonSummary[];
        game: GameState;
    }

    let { addons, game }: Props = $props();
</script>

<main>
    <div class="addons">
        {#each addons as addon (addon.id)}
            <Addon {addon} active={game.hasAddon(addon)} toggle={game.toggleAddon} />
        {/each}
    </div>
    <div>
        <Button onclick={() => game.loadCards()}>Continue</Button>
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
