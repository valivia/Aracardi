<script lang="ts">
    import type { Player } from "lib/player.svelte";
    import { DeleteIcon } from "lib/icons";

    interface Props {
        player: Player;
        active?: boolean;
        onDelete?: () => void;
    }

    let { player, active = false, onDelete }: Props = $props();
</script>

<button class="player" id={player.htmlId} class:active disabled={onDelete == undefined} onclick={onDelete}>
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

        &.active {
            color: var(--theme-accent);
        }

        &:focus-visible {
            outline: none;
            > .avatar {
                @include focused;
            }
        }
    }

    .avatar,
    .delete {
        @include avatar();
    }

    .delete {
        display: none;
    }

    .player:hover:not(:disabled) {
        color: var(--theme-error);

        :global(> .avatar) {
            display: none;
        }

        .delete {
            display: flex;
        }
    }
</style>
