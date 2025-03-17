<script lang="ts">
    import Donation from "components/Donation.svelte";
    import Header from "components/Header.svelte";
    import Button from "components/input/Button.svelte";
    import Toggle from "components/input/Toggle.svelte";
    import ThemeSelect from "components/ThemeSelect.svelte";
    import { type GameController } from "lib/game.svelte";

    interface Props {
        game: GameController;
        onchange: () => void;
    }

    let { game = $bindable(), onchange }: Props = $props();
</script>

<div class="layout">
    <Header title="Settings" />

    <main>
        <label>
            <span>Theme</span>
            <ThemeSelect />
        </label>
        <Toggle bind:checked={game.settings.allowNsfw} {onchange}>Allow NSFW</Toggle>
        <Toggle bind:checked={game.settings.allowDuplicates} {onchange}>Allow duplicates</Toggle>
        <Toggle bind:checked={game.settings.loadImages}>Load images</Toggle>
        {#if game.isOngoing}
            <dl>
                <dt>Cards</dt>
                <dd
                    class="animatedNumber"
                    style="--animatedNumber: {game.cards.length}"
                    aria-label="Card count: {game.cards.length}"
                >
                    /{game.cards.length + game.disabledCards.length}
                </dd>
                <dt>Players</dt>
                <dd>{game.players.length}</dd>
            </dl>
        {/if}
        <Donation />
    </main>

    <nav>
        <Button onclick={() => (game.settingsOpen = false)}>Done</Button>
    </nav>
</div>

<style lang="scss">
    @use "./layout.scss" as *;
    @use "/src/styles/abstracts" as *;

    main {
        position: relative;
        padding: 1rem;
        height: 100%;

        display: flex;
        flex-direction: column;
        justify-content: center;
        gap: 1.5rem;
    }

    label {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    dl {
        display: grid;
        grid-template-columns: min-content 1fr;
        gap: 0.5rem;

        dt {
            font-weight: bold;
        }
    }

    .animatedNumber {
        @include animatedNumber;
    }
</style>
