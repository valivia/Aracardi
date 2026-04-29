<script lang="ts">
    import { beforeNavigate } from "$app/navigation";
    import ActiveCard from "components/game/ActiveCard.svelte";
    import CardElement from "components/game/Card.svelte";
    import { Player } from "lib/player.svelte";
    import PlayerElement from "components/game/Player.svelte";
    import { WebsocketClient } from "lib/websocket.svelte";
    import { IncomingMessageTopic, OutgoingMessageTopic } from "lib/protocol.js";
    import { CardController } from "lib/card.svelte.js";
    import { useSettings } from "lib/settingsContext.js";
    import { onDestroy } from "svelte";
    import { HAS_TRIED_THEMES_KEY } from "components/theme.js";
    import type { Unsubscriber } from "svelte/store";
    import TextButton from "components/input/TextButton.svelte";

    const { data } = $props();

    let { settings } = useSettings();

    let socket = $state<WebsocketClient>(new WebsocketClient(data.lobby));
    socket.connectToSocket().catch(() => null);

    // Connection
    let hostConnected = $state(false);

    let currentCard: CardController | null = $state(null);
    let activeCards = $state<CardController[]>([]);

    let currentPlayerId = $state("");
    let players = $state<Player[]>([]);
    let currentPlayer = $state<Player>();

    let settingUnsubscriber: Unsubscriber | undefined;

    $inspect(socket.socketState);

    socket.onReady(() => {
        // Client settings
        if (settingUnsubscriber) settingUnsubscriber();
        settingUnsubscriber = settings.subscribe((value) => {
            socket.send(OutgoingMessageTopic.ClientUpdate, {
                theme: localStorage.getItem(HAS_TRIED_THEMES_KEY) === "true" ? value.theme : undefined,
                loadImages: value.loadImages,
                allowNsfw: value.allowNsfw,
            });
        });
    });

    socket.onMessage(IncomingMessageTopic.GameUpdate, (payload) => {
        if (payload.currentPlayerId) currentPlayerId = payload.currentPlayerId;
        if (payload.players) {
            players = payload.players.map((player) => {
                let result = new Player(player.name, player.avatar);
                result.id = player.id;
                return result;
            });
        }
        currentPlayer = players.find((p) => p.id === currentPlayerId);
        if (payload.activeCards)
            activeCards = payload.activeCards.map((card) =>
                CardController.fromSpectatorCard(card, currentPlayer?.name || "???"),
            );
        if (payload.currentCard) {
            currentCard = CardController.fromSpectatorCard(payload.currentCard, currentPlayer?.name || "???");
        }
        if (typeof payload.hostConnected == "boolean") {
            hostConnected = payload.hostConnected;
        }
    });

    socket.onClose(() => {
        console.log("closed");
    });

    // TODO: is redundant?
    beforeNavigate(() => {
        socket?.close();
    });

    onDestroy(() => {
        socket?.close();
        settingUnsubscriber?.();
    });
</script>

<svelte:document on:visibilitychange={() => socket.onVisibilityChange()} />

<div class="layout">
    <aside class="players">
        <div class="playerList">
            {#each players as player (player.id)}
                {@const active = currentPlayerId === player.id}
                <PlayerElement {player} {active} />
            {/each}
        </div>
    </aside>

    <main class="game">
        {#if socket.socketState.reason?.canReconnect}
            <TextButton onclick={() => socket.restartConnectionCycle()}>Retry</TextButton>
        {/if}
        {socket.statusString}
        {#if currentCard}
            <CardElement card={currentCard} loadImage={$settings.loadImages} />
        {/if}
    </main>

    <aside class="active">
        {#each activeCards as card (card.instanceId)}
            <ActiveCard {card} />
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
