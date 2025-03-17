<script lang="ts">
    import { PUBLIC_MINIMUM_PLAYER_COUNT } from "$env/static/public";
    import { avatars, type Avatar } from "assets/avatars/avatars.svelte";
    import Header from "components/Header.svelte";
    import Button from "components/input/Button.svelte";
    import Tag from "components/Tag.svelte";
    import { type GameController, GameStage } from "lib/game.svelte";
    import { DeleteIcon, UserIcon } from "lib/icons";
    import { Player } from "lib/player.svelte";

    // Props
    interface Props {
        game: GameController;
    }

    let { game }: Props = $props();

    // State
    type AvatarPlayerLink = Avatar & { player?: Player };
    let selectedAvatarName = $state(avatars[Math.floor(Math.random() * avatars.length)].name);
    let selectedAvatar: AvatarPlayerLink = $derived.by(() => {
        return avatarPlayerLink.find((avatar) => avatar.name === selectedAvatarName) ?? avatarPlayerLink[0];
    });

    let value = $state("");

    let avatarPlayerLink: AvatarPlayerLink[] = $derived.by(() => {
        return avatars.map((avatar) => {
            const player = game.players.find((player) => player.avatar.name === avatar.name);
            return { ...avatar, player };
        });
    });

    function setRandomAvatar() {
        const availableAvatars = avatarPlayerLink.filter((avatar) => {
            return avatar.player === undefined && avatar !== selectedAvatar;
        });

        if (availableAvatars.length === 0) {
            return;
        }

        selectAvatar(availableAvatars[Math.floor(Math.random() * availableAvatars.length)]);
    }

    function onSubmit(event: Event) {
        event.preventDefault();

        const name = value.trim();
        if (name.length < 3) return;

        const avatar = avatars.find((avatar) => avatar.name === selectedAvatar.name);
        if (!avatar) return;

        let player;
        if (selectedAvatar.player) {
            player = selectedAvatar.player;
            player.name = name;
        } else {
            player = new Player(name, avatar);
        }

        game.upsertPlayer(player);

        if (game.players.length < avatarPlayerLink.length) {
            setRandomAvatar();
        }
    }

    function removePlayer() {
        if (!selectedAvatar.player) return;
        game.removePlayer(selectedAvatar.player);
        value = "";
        setRandomAvatar();
    }

    function selectAvatar(avatar: AvatarPlayerLink) {
        if (avatar.player) {
            value = avatar.player.name;
        } else if (selectedAvatar.player) {
            value = "";
        }

        selectedAvatarName = avatar.name;
    }

    function loadPlayers() {
        game.restorePlayers();
        value = selectedAvatar.player?.name ?? "";
    }

    const hasEnoughPlayers = $derived.by(() => game.players.length >= Number(PUBLIC_MINIMUM_PLAYER_COUNT));
</script>

<div class="layout">
    <Header title="Players" subtitle="Add at least {PUBLIC_MINIMUM_PLAYER_COUNT} players to begin!" />

    <main>
        <!-- Selector -->
        <section class="selector">
            <!-- Add player -->
            <form onsubmit={onSubmit}>
                <input type="text" placeholder="Player name" bind:value minlength="3" maxlength="20" required />
                <section>
                    <Button type="submit" disabled={selectedAvatar.player?.name === value}>
                        {selectedAvatar.player ? "Update" : "Add"}
                    </Button>
                    {#if selectedAvatar.player !== undefined}
                        <Button type="button" color="danger" onclick={removePlayer} aria-label="Remove player">
                            <DeleteIcon />
                        </Button>
                    {/if}
                </section>
            </form>

            <!-- Current avatar -->
            {#key selectedAvatar.name}
                <button class="activeAvatar" onclick={setRandomAvatar} aria-label="Randomize avatar">
                    <selectedAvatar.element />
                </button>
            {/key}

            <!-- Details -->
            <dl>
                <dt>Name</dt>
                <dd>{selectedAvatar.name}</dd>
                <dt>Creator{selectedAvatar.authors.length > 1 ? "s" : ""}</dt>
                <dd>{selectedAvatar.authors.join(", ")}</dd>
            </dl>
        </section>

        <!-- Avatars -->
        <section class="avatars">
            {#each avatarPlayerLink as avatar}
                {@const selected = selectedAvatar.name === avatar.name}
                {@const active = avatar.player !== undefined}
                <div class="player" class:active class:selected>
                    <button
                        class="avatar"
                        onclick={() => selectAvatar(avatar)}
                        aria-label="Select {avatar.name} avatar for {active ? "Editing" : "Adding"}"
                    >
                        <avatar.element />
                    </button>
                    <div class="name">{avatar.player?.name}</div>
                </div>
            {/each}
            {#if game.hasPreviousPlayers && game.players.length === 0}
                <button onclick={loadPlayers}>Load previous Players...</button>
            {/if}
        </section>
    </main>

    <!-- Controls -->
    <nav>
        <section class="summary">
            <Tag icon={UserIcon}>{game.players.length}{hasEnoughPlayers ? "" : ` / ${PUBLIC_MINIMUM_PLAYER_COUNT}`}</Tag
            >
        </section>
        {#if !game.isOngoing}
            <Button variant="secondary" onclick={() => game.setStage(GameStage.addonSetup)}>Back</Button>
        {/if}
        <Button onclick={() => game.setStage(GameStage.game)} disabled={!hasEnoughPlayers}>
            {game.isOngoing ? "Done" : "Start Game"}
        </Button>
    </nav>
</div>

<style lang="scss">
    @use "styles/abstracts" as *;
    @use "./layout.scss" as *;

    $clampFactor: 10vw;

    main {
        gap: 1rem;
        width: min(100%, 80ch);
        display: grid;
        grid-template-rows: auto auto auto;
        grid-template-areas: "selector" "avatars" "controls";
        height: min-content;
    }

    .activeAvatar,
    .avatar {
        @include avatar();
    }

    button {
        color: currentColor;
        background: none;
        border: none;
        cursor: pointer;

        &:hover,
        &:focus-visible {
            color: var(--theme-accent);
        }
    }

    .selector {
        grid-area: selector;
        display: grid;
        gap: 1rem;
        grid-template-columns: 1fr 1fr;
        align-items: end;
        justify-items: center;

        @include large() {
            grid-template-columns: 1fr 7rem 1fr;
        }

        form {
            width: min(100%, 26ch);
            display: flex;
            gap: 0.5rem;
            flex-direction: column;

            input {
                padding: 0.5rem;
                border: none;
                color: currentColor;
                border-bottom: var(--border-width) solid var(--theme-text);
                background-color: transparent;

                &:focus-visible {
                    outline: none;
                    border-color: var(--theme-accent);
                }
            }

            section {
                display: flex;
                gap: 0.5rem;

                :global(button) {
                    font-size: 0.8rem;
                }

                :global(:first-child) {
                    flex: 1;
                }
            }
        }

        .activeAvatar {
            width: clamp(5rem, $clampFactor, 7rem);
            animation: spin 200ms forwards ease-in-out;
        }

        @keyframes spin {
            from {
                transform: rotate(-170deg);
            }
            to {
                transform: scale(1) rotate(0deg);
            }
        }

        dl {
            display: none;
            font-size: 0.9rem;
            text-align: center;

            @include large() {
                display: block;
            }

            dt {
                font-weight: bold;
                margin-top: 0.25rem;
            }
        }
    }

    .avatars {
        grid-area: avatars;
        font-size: clamp(3rem, $clampFactor, 5rem);
        border: var(--border-width) solid var(--theme-text);
        border-radius: var(--border-radius);

        width: 100%;
        padding: 0.3em;

        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(1em, 1fr));
        justify-content: center;
        gap: 0.15em;

        color: color-mix(in oklab, var(--theme-text), 20% black);

        .player {
            text-align: center;
            transition: transform 200ms ease-in-out;
            transform: translate();

            &.active {
                color: var(--theme-text);
            }

            &.selected {
                color: var(--theme-accent);
                transform: translateY(-0.1em);
            }

            .avatar {
                width: 100%;
                @include avatar();
            }

            .name {
                overflow: hidden;
                font-size: clamp(0.8rem, 2vw, 1rem);
                height: 1lh;
            }
        }
    }

    .summary {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
    }
</style>
