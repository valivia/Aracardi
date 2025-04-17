<script lang="ts" module>
    export const themes = {
        system: {
            displayName: "System",
        },
        light: {
            displayName: "Light",
        },
        dark: {
            displayName: "Dark",
        },
        purple: {
            displayName: "Purple",
        },
        artDeco: {
            displayName: "Art Deco",
        },
        paper: {
            displayName: "Paper",
        },
        red: {
            displayName: "Red",
        },
        pain: {
            displayName: "Pain",
        },
    };
</script>

<style lang="scss">
    @use "styles/themes" as *;

    :global(body[data-theme="dark"]) {
        @include darkTheme;
    }

    :global(body[data-theme="light"]) {
        @include lightTheme;
    }

    :global(body[data-theme="purple"]) {
        --theme-text: hsl(272.19deg 100% 12.34%);
        --theme-accent: hsl(275.1deg 100% 50%);
        --theme-primary: hsl(283.44deg 100% 87.67%);
        --theme-secondary: hsl(0, 0%, 95%);
        --theme-error: hsl(0, 64%, 59%);
    }

    :global(body[data-theme="artDeco"]) {
        --theme-text: hsl(39deg 78.34% 62.75%);
        --theme-accent: hsl(33, 90%, 54%);
        --theme-primary: hsl(171.9deg 52.59% 10.12%);
        --theme-secondary: hsl(172deg 83.93% 3.99%);
        --theme-error: hsl(6, 74%, 59%);
    }

    :global(body[data-theme="paper"]) {
        --theme-text: hsl(0, 0%, 14%);
        --theme-accent: hsl(0, 0%, 14%);
        --theme-primary: hsl(0, 0%, 79%);
        --theme-secondary: hsl(0, 0%, 56%);
        --theme-error: hsl(0, 15%, 14%);

        :global(#currentCard img) {
            filter: grayscale(1) brightness(0.8);
        }
    }

    :global(body[data-theme="red"]) {
        --theme-text: hsl(0deg 100% 60.6%);
        --theme-accent: hsl(0deg 88.14% 67.99%);
        --theme-primary: hsl(0deg 0% 4.3%);
        --theme-secondary: hsl(0, 0%, 10%);
        --theme-error: hsl(0deg 100% 60.6%);

        :global(#currentCard img) {
            filter: sepia(100%) saturate(200%) brightness(70%) hue-rotate(330deg);
        }
    }

    :global(body[data-theme="pain"]) {
        --theme-text: hsl(130.18deg 66.89% 83.14%);
        --theme-accent: hsl(105.39deg 100% 50%);
        --theme-primary: hsl(271.98deg 100% 50%);
        --theme-secondary: hsl(261.04deg 25.36% 48.3%);
        --theme-error: hsl(0, 57%, 51%);
    }
</style>
