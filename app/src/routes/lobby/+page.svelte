<script lang="ts">
    import { avatars } from "assets/avatars/avatars.svelte";
    import { PlusIcon, ShuffleIcon } from "svelte-feather-icons";
    import Card from "components/game/Card.svelte";
    import ActiveCard from "components/game/ActiveCard.svelte";
    import Player from "components/game/Player.svelte";
</script>

<div class="layout">
    <aside class="players">
        <div class="playerList">
            <button class="playerButton">
                <PlusIcon size="50%" strokeWidth={1} />
            </button>
            {#each avatars as avatar}
                <Player
                    player={{
                        name: avatar.name,
                        avatar: avatar.name,
                        host: false,
                        id: "",
                    }}
                    isDeleteable={true}
                />
            {/each}
            <button class="playerButton">
                <ShuffleIcon />
            </button>
        </div>
    </aside>

    <main class="game">
        <Card
            name="Test"
            text="Test"
            image="https://cdn.discordapp.com/attachments/798915150445936750/1194336323096559756/20231213_192137.jpg"
        />
    </main>

    <aside class="active">
        <ActiveCard text="Test" />
        <ActiveCard text="Test" />
        <ActiveCard text="Test" />
        <ActiveCard text="Test" />
        <ActiveCard text="Test" />
        <ActiveCard text="Test" />
        <ActiveCard text="Test" />
    </aside>
</div>

<style lang="scss">
    @use "styles/abstracts" as *;

    .layout {
        display: grid;
        grid-template-columns: 1fr;
        grid-template-rows: 1fr 3fr 1fr;
        grid-template-areas: "players" "game" "active";

        @include large() {
            grid-template-columns: 1fr 3fr 1fr;
            grid-template-rows: 1fr;
            grid-template-areas: "players game active";
        }
    }

    $player-size: clamp(4rem, 5vw, 6rem);

    .players {
        grid-area: players;
        max-width: 100vw; // TODO: ??

        @include large() {
            max-width: unset;
            max-height: 60vh;
        }

        & .playerList {
            position: relative;
            padding: 1rem;

            display: grid;
            grid-auto-columns: $player-size;
            grid-template-rows: auto;
            grid-auto-flow: column;
            align-items: start;
            grid-gap: 1rem;
            overflow: scroll auto;

            @include large() {
                padding: 1rem;
                grid-template-columns: $player-size;
                grid-auto-rows: auto;
                grid-auto-flow: row;
                max-height: 100%;
                width: min-content;
                overflow: hidden scroll;

                --gradient-orientation: 0deg;
            }
        }
    }

    .playerButton {
        @include avatar;
        background-color: transparent;
        color: currentColor;

        &:hover {
            color: var(--theme-accent);
        }
    }

    .game {
        grid-area: game;

        display: flex;
        justify-content: center;
        align-items: center;
    }

    .active {
        grid-area: active;
        display: grid;
        gap: 1rem;

        $activeCardHeight: clamp(4rem, 5vw, 6rem);
        $activeCardWidth: clamp(6rem, 5vw, 8rem);

        grid-template-rows: $activeCardHeight;
        grid-auto-columns: $activeCardWidth;
        grid-auto-flow: column;
        overflow: scroll hidden;

        @include large() {
            grid-template-columns: 1fr;
            grid-auto-rows: 1fr;
            grid-auto-flow: row;
            overflow: hidden scroll;
        }
    }
</style>
