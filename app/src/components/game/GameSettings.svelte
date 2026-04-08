<script lang="ts">
    import type { GameController } from "lib/game.svelte";

    interface Props {
        game: GameController;
    }

    let { game = $bindable() }: Props = $props();
</script>

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

<style lang="scss">
    @use "/src/styles/abstracts" as *;

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
