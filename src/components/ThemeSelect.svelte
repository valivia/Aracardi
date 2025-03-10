<script lang="ts" module>
    export const themes = {
        system: {
            class: undefined,
            displayName: "System",
        },
        light: {
            class: "themeLight",
            displayName: "Light",
        },
        dark: {
            class: "themeDark",
            displayName: "Dark",
        },
        purple: {
            class: "themePurple",
            displayName: "Purple",
        },
        artDeco: {
            class: "themeArtDeco",
            displayName: "Art Deco",
        },
    };

    export function setTheme(themeKey: keyof typeof themes) {
        const allClasses = Object.values(themes)
            .map((theme) => theme.class)
            .filter((theme) => typeof theme === "string");

        document.body.classList.remove(...allClasses);

        const theme = themes[themeKey];
        if (!theme) return;

        if (theme.class === undefined) {
            localStorage.removeItem("theme");
        } else {
            localStorage.setItem("theme", themeKey);
            document.body.classList.add(theme.class);
        }
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

        &:hover {
            border-color: var(--theme-accent);
        }
    }
</style>
