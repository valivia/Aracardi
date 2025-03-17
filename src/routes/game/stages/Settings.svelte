<script lang="ts">
    import Donation from "components/Donation.svelte";
    import Header from "components/Header.svelte";
    import Button from "components/input/Button.svelte";
    import Toggle from "components/input/Toggle.svelte";
    import ThemeSelect from "components/ThemeSelect.svelte";
    import { type GameController } from "lib/game.svelte";

    interface Props {
        game: GameController;
    }

    let { game = $bindable() }: Props = $props();
</script>

<div class="layout">
    <Header title="Settings" />

    <main>
        <label>
            <span>Theme</span>
            <ThemeSelect />
        </label>
        {#if !game.isOngoing}
            <Toggle bind:checked={game.settings.allowNsfw}>Allow NSFW</Toggle>
            <Toggle bind:checked={game.settings.allowDuplicates}>Allow duplicates</Toggle>
        {/if}
        <Toggle bind:checked={game.settings.loadImages}>Load images</Toggle>

        <Donation />
    </main>

    <nav>
        <Button onclick={() => (game.settingsOpen = false)}>Done</Button>
    </nav>
</div>

<style lang="scss">
    @use "./layout.scss" as *;

    main {
        position: relative;
        padding: 1rem;
        height: 100%;

        display: flex;
        flex-direction: column;
        justify-content: center;
        gap: 1.5rem;
    }

    label {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
</style>
