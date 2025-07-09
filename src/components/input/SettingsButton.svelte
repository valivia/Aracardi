<script lang="ts">
    import type { GameController } from "lib/game.svelte";
    import { SettingsIcon } from "components/icons";

    interface Props {
        game: GameController;
    }

    let { game }: Props = $props();
</script>

<button
    onclick={() => (game.settingsOpen = !game.settingsOpen)}
    aria-label={`${game.settingsOpen ? "Close" : "Open"} settings menu`}><SettingsIcon /></button
>

<style lang="scss">
    button {
        font-size: clamp(1rem, 2vw, 2rem);

        position: fixed;
        color: currentColor;
        top: 1em;
        right: 1em;
        width: 1em;
        height: 1em;
        z-index: 1000;
        background-color: transparent;
        cursor: pointer;
        border: none;

        border-radius: 100vw;

        // tranisition color and transform
        transition-property: color, transform;
        transition-duration: 0.2s;
        transition-timing-function: ease-in-out;

        &:hover,
        &:focus-visible {
            color: var(--theme-accent);
            transform: rotate(-60deg);
        }

        &:focus-visible {
            outline: var(--outline-focus);
            outline-offset: var(--outline-focus-offset);
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
