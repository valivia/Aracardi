import { themes } from "../lib/themes";

export const HAS_TRIED_THEMES_KEY = "hasTriedThemes";

export function applyTheme(themeKey: keyof typeof themes) {
    const theme = themes[themeKey];
    if (!theme || themeKey === "system") {
        document.body.removeAttribute("data-theme");
        return;
    }

    document.body.setAttribute("data-theme", themeKey);
    localStorage.setItem(HAS_TRIED_THEMES_KEY, "true");
}

export function getTheme(): keyof typeof themes | undefined {
    try {
        const settings = JSON.parse(localStorage.getItem("settings") ?? "null");
        const theme = settings?.["theme"];
        return theme in themes ? theme : undefined;
    } catch {
        return undefined;
    }
}

export function syncTheme() {
    const theme = getTheme();
    if (theme) applyTheme(theme);
}
