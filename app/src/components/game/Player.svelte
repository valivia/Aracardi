<script lang="ts">
    import type { Player } from "$lib/lobby/player";
    import Avatar from "components/avatar.svelte";
    import { DeleteIcon } from "lib/icons";

    export let player: Player;
    export let isDeleteable = true;
</script>

<button class="player" disabled={!isDeleteable}>
    <Avatar name={player.avatar} />
    <div class="delete">
        <DeleteIcon width="40%" height="40%"/>
    </div>

    <span class="name">{player.name}</span>
</button>

<style lang="scss">
    @use "styles/abstracts" as *;

    .player {
        color: currentColor;
        border: none;
        background-color: transparent;
    }

    .delete {
        @include avatar();
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
