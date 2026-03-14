<script lang="ts">
    import { beforeNavigate } from "$app/navigation";
    import ActiveCard from "components/game/ActiveCard.svelte";
    import CardElement from "components/game/Card.svelte";
    import { Player } from "lib/player.svelte";
    import PlayerElement from "components/game/Player.svelte";
    import { WebsocketClient } from "lib/websocket";
    import type { GameCard, GameUpdate } from "lib/protocol.js";
    import { CardController } from "lib/card.svelte.js";

    const { data } = $props();

    let socket = $state<WebsocketClient | null>();

    beforeNavigate(() => {
        socket?.close();
    });

    // Connection
    let hostConnected = $state(false);
    let serverConnected = $state(false);

    let currentCard: CardController | null = $state(null);
    let activeCards = $state<GameCard[]>([]);

    let currentPlayer = $state("");
    let players = $state<Player[]>([]);

    async function connect() {
        socket = await WebsocketClient.connectToSession(data.lobby);
        socket?.onMessage((type, payload) => {
            serverConnected = true;
            if (type == "update") {
                let gameUpdate: GameUpdate = JSON.parse(payload);
                if (gameUpdate.activeCards) activeCards = gameUpdate.activeCards;
                if (gameUpdate.currentPlayerId) currentPlayer = gameUpdate.currentPlayerId;
                if (gameUpdate.currentCard) {
                    currentCard = CardController.fromSpectatorCard(
                        gameUpdate.currentCard,
                        players.find((p) => p.id === currentPlayer)?.name ?? "???",
                    );
                }
                if (typeof gameUpdate.hostConnected == "boolean") {
                    hostConnected = gameUpdate.hostConnected;
                }
                if (gameUpdate.players) {
                    players = gameUpdate.players.map((player) => {
                        let result = new Player(player.name, player.avatar);
                        result.id = player.id;
                        return result;
                    });
                }
            }
        });

        socket?.onClose(() => {
            serverConnected = false;
        });
    }

    connect();
</script>

<div class="layout">
    <aside class="players">
        <div class="playerList">
            {#each players as player}
                {@const active = currentPlayer === player.id}
                <PlayerElement {player} {active} onDelete={undefined} />
            {/each}
        </div>
    </aside>

    <main class="game">
        <span>host {hostConnected ? "connected" : "unavailable"}</span>
        <span>server {serverConnected ? "connected" : "unavailable"}</span>
        {#if currentCard}
            <CardElement card={currentCard} onclick={undefined} loadImage={true} />
            <!--Make loadimage dynamic-->
        {/if}
    </main>

    <aside class="active">
        {#each activeCards as card}
            <ActiveCard {card} onclick={undefined} />
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

        @include defaultOrientation() {
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

        @include defaultOrientation() {
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

        @include defaultOrientation() {
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

            @include defaultOrientation() {
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
        padding-inline: 1.5em;

        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 1em;

        @include defaultOrientation() {
            height: 100%;
            overflow: hidden;
        }

        section {
            display: flex;
            gap: 1em;
        }
    }

    // Active cards area
    .active {
        grid-area: active;

        grid-template-rows: 10em;
        grid-auto-columns: 14em;

        @include defaultOrientation() {
            grid-template-columns: min(30ch, 100%);
            grid-auto-rows: min-content;
            grid-template-rows: unset;
            padding: 0;
            padding-right: 1em;
        }
    }
</style>
