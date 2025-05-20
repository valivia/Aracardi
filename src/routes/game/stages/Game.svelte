<script lang="ts">
    import Card from "components/game/Card.svelte";
    import ActiveCard from "components/game/ActiveCard.svelte";
    import Player from "components/game/Player.svelte";
    import { PlusIcon, ShuffleIcon } from "components/icons";
    import { GameStage, GameController } from "lib/game.svelte";
    import { PUBLIC_MINIMUM_PLAYER_COUNT } from "$env/static/public";
    import Button from "components/input/Button.svelte";

    interface Props {
        game: GameController;
    }

    let { game }: Props = $props();

    const minPlayers = Number(PUBLIC_MINIMUM_PLAYER_COUNT);

    $effect(() => {
        const player = document.getElementById(game.currentPlayer.htmlId);

        if (player) {
            player.scrollIntoView({ behavior: "smooth", block: "center" });
        }
    });
</script>

<div class="layout">
    <aside class="players">
        <div class="playerList">
            <button
                class="playerButton"
                onclick={() => game.setStage(GameStage.playerSetup)}
                aria-label="Add player"
                title="Add player"
            >
                <PlusIcon width="50%" height="50%" />
            </button>
            <button
                class="playerButton"
                onclick={() => game.shufflePlayers()}
                aria-label="Shuffle players"
                title="Shuffle players"
            >
                <ShuffleIcon width="35%" height="35%" />
            </button>
            {#each game.players as player}
                {@const active = game.currentPlayer.id === player.id}
                {@const canDelete = game.players.length > minPlayers && !active}
                {@const onDelete = canDelete
                    ? () => confirm(`Are you sure you want to remove ${player.name}?`) && game.removePlayer(player)
                    : undefined}
                <Player {player} {active} {onDelete} />
            {/each}
        </div>
    </aside>

    <main class="game">
        {#if game.currentCard}
            <Card card={game.currentCard} onclick={() => game.nextTurn()} loadImage={game.settings.loadImages} />
        {/if}
        <section>
            <Button onclick={() => game.nextTurn()}>Next card</Button>
        </section>
    </main>

    <aside class="active">
        {#each game.activeCards as card}
            <ActiveCard {card} onclick={() => game.deleteActiveCard(card, true)} />
        {/each}
    </aside>
</div>

<style lang="scss">
    @use "styles/abstracts" as *;

    .layout {
        display: grid;
        grid-template-columns: 1fr;
        grid-template-rows: min-content auto min-content;
        grid-template-areas: "players" "game" "active";
        align-items: center;
        justify-items: center;
        gap: 1rem;
        height: 100%;
        overflow: hidden auto;

        @include large() {
            grid-template-columns: 1fr 3fr 1fr;
            grid-template-rows: 1fr;
            grid-template-areas: "players game active";
            align-items: unset;
            justify-items: unset;
            height: unset;
        }
    }

    .players,
    .active {
        max-width: 90vw;

        @include large() {
            max-width: unset;
            max-height: 70dvh;
        }
    }

    // Lists
    .playerList,
    .active {
        overflow: auto hidden;
        display: grid;
        grid-auto-flow: column;
        padding: 0.5rem;
        gap: 1rem;

        @include large() {
            overflow: hidden auto;
            grid-auto-flow: row;
            padding-block: 0.5rem;
        }
    }

    // Player area

    $player-size: clamp(4rem, 5vw, 6rem);

    .players {
        grid-area: players;

        & .playerList {
            position: relative;

            grid-auto-columns: $player-size;
            grid-template-rows: auto;
            align-items: start;

            @include large() {
                grid-template-columns: $player-size;
                grid-auto-rows: auto;
                max-height: 100%;
                padding-inline: 1rem;
                width: min-content;
            }
        }
    }

    .playerButton {
        @include avatar;
        background-color: transparent;
        color: currentColor;

        &:focus-visible,
        &:hover {
            color: var(--theme-accent);
            outline: var(--border-width) solid currentColor;
            outline-offset: 2px;
        }
    }

    // Main area
    .game {
        grid-area: game;
        padding-inline: 2rem;

        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        @include large() {
            height: 100%;
            overflow: hidden;
        }

        section {
            display: flex;
            gap: 1rem;
            margin-top: 0.5rem;
        }
    }

    // Active cards area
    .active {
        grid-area: active;

        grid-template-rows: 10rem;
        grid-auto-columns: 14rem;
        justify-content: right;

        @include large() {
            grid-template-columns: min(30ch, 100%);
            grid-auto-rows: min-content;
            grid-template-rows: unset;
            padding: 0;
            padding-right: 1rem;
        }
    }
</style>
