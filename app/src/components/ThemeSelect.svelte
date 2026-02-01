<script lang="ts" module>
    import { themes } from "../lib/themes";

    export function setTheme(themeKey: keyof typeof themes) {
        const theme = themes[themeKey];
        if (!theme || themeKey === "system") {
            document.body.removeAttribute("data-theme");
            localStorage.removeItem("theme");
            return;
        }

        document.body.setAttribute("data-theme", themeKey);
        localStorage.setItem("theme", themeKey);
    }

    export function getTheme() {
        const key = localStorage.getItem("theme");
        if (key && key in themes) return key as keyof typeof themes;
        return undefined;
    }

    export function syncTheme() {
        const theme = getTheme();
        if (theme) setTheme(theme);
    }
</script>

<script lang="ts">
    let theme: keyof typeof themes = $state(getTheme() || "system");
</script>

<select bind:value={theme} onchange={() => setTheme(theme)}>
    {#each Object.entries(themes) as [key, value]}
        <option value={key}>{value.displayName}</option>
    {/each}
</select>

<style lang="scss">
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
</style>
