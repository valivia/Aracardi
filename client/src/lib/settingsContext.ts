import { getContext, setContext } from "svelte";
import { writable, type Writable } from "svelte/store";
import type { Snippet } from "svelte";
import { applyTheme } from "components/theme";
import { themes } from "../lib/themes";

const KEY = Symbol("settings");

export interface Settings {
    theme: keyof typeof themes;
    allowNsfw: boolean;
    loadImages: boolean;
    allowDuplicates: boolean;
}

export const defaultSettings: Settings = {
    theme: "system",
    allowNsfw: true,
    loadImages: true,
    allowDuplicates: false,
};

function parseSettings(raw: unknown): Partial<Settings> {
    if (!raw || typeof raw !== "object") return {};
    const s = raw as Record<string, unknown>;
    return {
        ...(typeof s.theme === "string" && { theme: s.theme as keyof typeof themes }),
        ...(typeof s.allowNsfw === "boolean" && { allowNsfw: s.allowNsfw }),
        ...(typeof s.loadImages === "boolean" && { loadImages: s.loadImages }),
        ...(typeof s.allowDuplicates === "boolean" && { allowDuplicates: s.allowDuplicates }),
    };
}

export interface SettingsContext {
    open: () => void;
    close: () => void;
    toggle: () => void;
    isOpen: Writable<boolean>;

    settings: Writable<Settings>;

    setExtras: (snippet: Snippet | null) => void;
    extras: Writable<Snippet | null>;
}

export function createSettingsContext(): SettingsContext {
    const isOpen = writable(false);
    const extras = writable<Snippet | null>(null);
    const settings = writable<Settings>(defaultSettings);

    // Load settings
    const settingsJson = localStorage.getItem("settings");
    if (settingsJson) {
        try {
            const parsedSettings = parseSettings(JSON.parse(settingsJson));
            console.info("- Loaded settings");
            settings.update((current) => ({ ...current, ...parsedSettings }));
        } catch {
            console.warn("Invalid saved settings");
            localStorage.removeItem("settings");
        }
    }

    let previousSettings: Settings | undefined;

    // Auto save
    settings.subscribe((update) => {
        if (previousSettings === undefined) {
            previousSettings = { ...update };
            return;
        }

        if (previousSettings.theme !== update.theme) {
            applyTheme(update.theme);
        }

        console.log("- Settings saved");
        localStorage.setItem("settings", JSON.stringify(update));
        previousSettings = { ...update };
    });

    const ctx: SettingsContext = {
        // State
        isOpen,
        open: () => isOpen.set(true),
        close: () => isOpen.set(false),
        toggle: () => isOpen.update((x) => !x),

        // Settings
        settings,

        // Extras
        setExtras: (s) => extras.set(s),
        extras,
    };

    setContext(KEY, ctx);
    return ctx;
}

export function useSettings(): SettingsContext {
    return getContext<SettingsContext>(KEY);
}
