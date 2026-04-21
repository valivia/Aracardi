<script lang="ts">
    import Links from "components/Links.svelte";
    import Header from "components/Header.svelte";
    import Button from "components/input/Button.svelte";
    import { useSettings } from "lib/settingsContext";
    import Toggle from "components/input/Toggle.svelte";
    import { themes } from "../lib/themes";

    const { close, extras, settings } = useSettings();
</script>

<dialog class="layout" open>
    <Header title="Settings" />

    <main>
        <label>
            <span>Theme</span>
            <select bind:value={$settings.theme}>
                {#each Object.entries(themes) as [key, value] (key)}
                    <option value={key}>{value.displayName}</option>
                {/each}
            </select>
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

    select {
        padding: 0.5rem;
        font-size: 1rem;
        border: 2px solid currentColor;
        border-radius: var(--border-radius);
        color: currentColor;
        background-color: var(--theme-primary);

        &:hover,
        &:focus-visible {
            outline-color: var(--theme-accent);
            border-color: var(--theme-accent);
        }
    }

    label {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
</style>
