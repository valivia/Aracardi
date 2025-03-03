<script lang="ts">
    import Addon from "components/Addon.svelte";
    import Button from "components/input/Button.svelte";
    import type { AddonSummary } from "lib/addon";
    import Tag from "components/Tag.svelte";
    import { CardsIcon } from "lib/icons";
    import AnchorButton from "components/input/AnchorButton.svelte";
    import { PUBLIC_MINIMUM_CARD_COUNT } from "$env/static/public";
    import type { GameController } from "lib/game.svelte";

    interface Props {
        addons: AddonSummary[];
        game: GameController;
    }

    let { addons, game }: Props = $props();

    let hasEnoughCards = $derived(game.cardCount >= Number(PUBLIC_MINIMUM_CARD_COUNT));
</script>

<main>
    <!-- Info -->
    <section class="info">
        <h2>Card packs</h2>
        <p>Choose which card packs you'd like to play with.</p>
    </section>

    <!-- Addons -->
    <div class="addons">
        {#each addons as addon (addon.id)}
            <Addon {addon} active={game.hasAddon(addon)} toggle={game.toggleAddon} />
        {/each}
    </div>

    <section class="summary">
        <Tag icon={CardsIcon}>{game.cardCount}{hasEnoughCards ? "" : ` / ${PUBLIC_MINIMUM_CARD_COUNT}`}</Tag>
    </section>

    <!-- Actions -->
    <section class="actions">
        <AnchorButton variant="secondary" href="/">Back</AnchorButton>
        <Button onclick={() => game.loadCards()} disabled={!hasEnoughCards}>Continue</Button>
    </section>
</main>

<style lang="scss">
    main {
        margin-inline: auto;
        width: min(100%, 80ch);
        padding: 1rem;

        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .info {
        display: flex;
        flex-direction: column;
        gap: 1rem;

        margin-inline: auto;
        text-align: center;
        max-width: min(100%, 40ch);

        font-size: clamp(0.8rem, 2vw, 1rem);

        h2 {
            font-size: 1.5em;
            font-weight: 600;
        }

        p {
            font-size: 1em;
        }
    }

    .addons {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        overflow-y: auto;
        max-height: 50vh;
        padding: 1rem;
    }

    .summary {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 1rem;
    }

    .actions {
        display: flex;
        justify-content: center;
        gap: 1rem;
    }
</style>
