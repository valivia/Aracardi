<script lang="ts">
    import type { Player } from "lib/player.svelte";
    import { DeleteIcon } from "components/icons";

    interface Props {
        player: Player;
        active?: boolean;
        onDelete?: () => void;
    }

    let { player, active = false, onDelete }: Props = $props();
</script>

<button
    class="player"
    id={player.htmlId}
    class:active
    disabled={onDelete == undefined}
    onclick={onDelete}
    aria-label="Remove player"
>
    <div class="avatar">
        <player.avatar.element />
    </div>
    <div class="delete">
        <DeleteIcon width="40%" height="40%" />
    </div>

    <span class="name">{player.name}</span>
</button>

<style lang="scss">
    @use "styles/abstracts" as *;

    .player {
        color: currentColor;
        border: none;
        background-color: transparent;

        display: flex;
        flex-direction: column;

        &.active {
            color: var(--theme-accent);
            .avatar {
                outline: var(--outline-focus);
                outline-offset: var(--outline-focus-offset);
            }
        }

        &:focus-visible {
            color: var(--theme-accent);
            outline: none;
        }
    }

    .avatar,
    .delete {
        @include avatar();
    }

    .delete {
        display: none;
    }

    .player:hover:not(:disabled),
    .player:focus-visible:not(:disabled) {
        color: var(--theme-error);
        cursor: pointer;

        :global(> .avatar) {
            display: none;
        }

        .delete {
            display: flex;
        }
    }

    .name {
        font-weight: 500;
        font-size: 0.9rem;
        padding-top: 0.3rem;
    }
</style>
