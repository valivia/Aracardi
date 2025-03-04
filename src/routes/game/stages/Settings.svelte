<script lang="ts">
    import Header from "components/Header.svelte";
    import Button from "components/input/Button.svelte";
    import Toggle from "components/input/Toggle.svelte";
    import { type GameController } from "lib/game.svelte";

    interface Props {
        game: GameController;
    }

    let { game = $bindable() }: Props = $props();
</script>

<div class="layout">
    <Header title="Settings" />

    <main>
        <Toggle bind:checked={game.settings.allowNsfw}>Allow NSFW</Toggle>
        <Toggle bind:checked={game.settings.allowDuplicates}>Allow duplicates</Toggle>
        <Toggle bind:checked={game.settings.loadImages}>Load images</Toggle>
    </main>

    <nav>
        <Button onclick={() => (game.settingsOpen = false)}>Done</Button>
    </nav>
</div>

<style lang="scss">
    @use "./layout.scss" as *;

    main {
        padding: 1rem;

        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }
</style>
