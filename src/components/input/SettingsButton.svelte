<script lang="ts">
    import type { GameController } from "lib/game.svelte";
    import { SettingsIcon } from "lib/icons";

    interface Props {
        game: GameController;
    }

    let { game }: Props = $props();
</script>

<button onclick={() => (game.settingsOpen = !game.settingsOpen)}><SettingsIcon /></button>

<style lang="scss">
    button {
        position: fixed;
        color: currentColor;
        top: 1rem;
        right: 1rem;
        width: 2rem;
        height: 2rem;
        z-index: 1000;
        background-color: transparent;
        cursor: pointer;
        border: none;

        // tranisition color and transform
        transition-property: color, transform;
        transition-duration: 0.2s;
        transition-timing-function: ease-in-out;

        &:hover {
            color: var(--color-accent);
            transform: rotate(-60deg);
        }

        &:active {
            transform: scale(0.9) rotate(-60deg);
        }

        :global(svg) {
            width: 100%;
            height: 100%;
        }
    }
</style>
