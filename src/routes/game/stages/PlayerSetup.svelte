<script lang="ts">
    import { avatars, type Avatar } from "assets/avatars/avatars.svelte";
    import Button from "components/input/Button.svelte";
    import { type GameController, GameStage } from "lib/game.svelte";
    import type { Player } from "lib/player.svelte";

    interface Props {
        game: GameController;
    }

    let { game }: Props = $props();

    type AvatarPlayerLink = Avatar & { playerName?: string };

    let selectedAvatar: AvatarPlayerLink = $state(avatars[0]);
    let value = $state("");

    let avatarPlayerLink: AvatarPlayerLink[] = $derived.by(() => {
        return avatars.map((avatar) => {
            const playerName = game.players.find((player) => player.avatar.name === avatar.name);
            return { ...avatar, playerName: playerName?.name };
        });
    });

    function setRandomAvatar() {
        const availableAvatars = avatarPlayerLink.filter(
            (avatar) => avatar.playerName === undefined && avatar !== selectedAvatar,
        );

        if (availableAvatars.length === 0) {
            return;
        }

        selectAvatar(availableAvatars[Math.floor(Math.random() * availableAvatars.length)]);
    }

    function onSubmit(event: Event) {
        event.preventDefault();
        const name = value.trim();
        if (name.length < 3) return;
        selectedAvatar.playerName = name;
        const player: Player = {
            name,
            avatar: selectedAvatar,
            id: Math.random().toString(36).slice(2),
        };

        game.upsertPlayer(player);

        setRandomAvatar();
    }

    function selectAvatar(avatar: AvatarPlayerLink) {
        if (selectedAvatar.playerName && selectedAvatar.playerName !== value) {
            alert("You have unsaved changes!");
            return;
        }
        selectedAvatar = avatar;
        value = avatar.playerName ?? "";
    }
</script>

<div class="wrap">
    <main>
        <!-- Selector -->
        <section class="selector">
            <!-- Add player -->
            <form onsubmit={onSubmit}>
                <input type="text" placeholder="Player name" bind:value minlength="3" maxlength="20" required />
                <Button type="submit">{selectedAvatar.playerName ? "Update" : "Add"}</Button>
            </form>

            <!-- Current avatar -->
            {#key selectedAvatar.name}
                <button class="activeAvatar" onclick={setRandomAvatar}>
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
                {@const active = avatar.playerName !== undefined}
                <div class="player" class:active class:selected>
                    <button class="avatar" onclick={() => selectAvatar(avatar)}>
                        <avatar.element />
                    </button>
                    <div class="name">{avatar.playerName}</div>
                </div>
            {/each}
        </section>

        <!-- Controls -->
        <section class="controls">
            {#if !game.isOngoing}
                <Button variant="secondary" onclick={() => game.setStage(GameStage.addonSetup)}>Back</Button>
            {/if}
            <Button onclick={() => game.setStage(GameStage.game)} disabled={game.players.length < 3}>
                {game.isOngoing ? "Done" : "Start Game"}
            </Button>
        </section>
    </main>
</div>

<style lang="scss">
    @use "styles/abstracts" as *;

    $clampFactor: 10vw;

    .wrap {
        display: grid;
        place-items: center;
        min-height: 100%;
    }

    main {
        margin-inline: auto;
        width: min(100%, 80ch);

        padding: 1rem;
        padding-top: 10dvh;

        display: flex;
        flex-direction: column;
        gap: 1rem;
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
    }

    .selector {
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
        font-size: clamp(3.5rem, $clampFactor, 5rem);
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

            &.active {
                color: var(--theme-text);
            }

            &.selected {
                color: var(--theme-accent);
                transform: scale(1.1);
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

    .controls {
        display: flex;
        gap: 1rem;
        justify-content: center;
    }
</style>
