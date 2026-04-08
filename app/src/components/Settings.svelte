<script lang="ts">
    import Links from "components/Links.svelte";
    import Header from "components/Header.svelte";
    import Button from "components/input/Button.svelte";
    import ThemeSelect from "components/ThemeSelect.svelte";
    import { useSettings } from "lib/settingsContext";
    import Toggle from "components/input/Toggle.svelte";

    const { close, extras, settings } = useSettings();
</script>

<dialog class="layout" open>
    <Header title="Settings" />

    <main>
        <label>
            <span>Theme</span>
            <ThemeSelect />
        </label>
        <Toggle bind:checked={$settings.allowNsfw}>Allow NSFW</Toggle>
        <Toggle bind:checked={$settings.loadImages}>Load images</Toggle>
        <Toggle bind:checked={$settings.allowDuplicates}>Allow duplicates</Toggle>
        {@render $extras?.()}
        <Links extended />
    </main>

    <nav>
        <Button onclick={close}>Done</Button>
    </nav>
</dialog>

<style lang="scss">
    @use "styles/layout.scss" as *;

    dialog {
        all: unset;
    }

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
