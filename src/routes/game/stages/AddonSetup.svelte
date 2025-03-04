<script lang="ts">
    import Addon from "components/Addon.svelte";
    import Button from "components/input/Button.svelte";
    import type { AddonSummary } from "lib/addon";
    import Tag from "components/Tag.svelte";
    import { CardsIcon } from "lib/icons";
    import AnchorButton from "components/input/AnchorButton.svelte";
    import { PUBLIC_MINIMUM_CARD_COUNT } from "$env/static/public";
    import { type GameController } from "lib/game.svelte";
    import Header from "components/Header.svelte";

    interface Props {
        addons: AddonSummary[];
        game: GameController;
    }

    let { addons, game }: Props = $props();

    let hasEnoughCards = $derived(game.cardCount >= Number(PUBLIC_MINIMUM_CARD_COUNT));
</script>

<div class="layout">
    <Header title="Card packs" subtitle="Choose which card packs you'd like to play with." />

    <main>
        <!-- Addons -->
        <div class="addons">
            {#each addons as addon}
                <Addon
                    {addon}
                    active={game.hasAddon(addon)}
                    allowNsfw={game.settings.allowNsfw}
                    toggle={game.toggleAddon}
                />
            {/each}
        </div>
    </main>

    <!-- Actions -->
    <nav>
        <section class="summary">
            <Tag icon={CardsIcon}>{game.cardCount}{hasEnoughCards ? "" : ` / ${PUBLIC_MINIMUM_CARD_COUNT}`}</Tag>
        </section>
        <AnchorButton variant="secondary" href="/">Back</AnchorButton>
        <Button onclick={() => game.loadCards()} disabled={!hasEnoughCards}>Continue</Button>
    </nav>
</div>

<style lang="scss">
    @use "./layout.scss" as *;

    main {
        display: grid;
        width: min(100%, 80ch);
        gap: 0.5rem;
        grid-template-rows: 1fr min-content;
        max-height: 100%;
    }

    .addons {
        display: flex;
        flex-direction: column;
        max-height: 50vh;
        gap: 1rem;
        padding: 1rem;
        margin-inline: -1rem;
        overflow-y: auto;
    }

    .summary {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
    }
</style>
