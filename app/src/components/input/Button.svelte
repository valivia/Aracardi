<script lang="ts">
    import type { HTMLButtonAttributes } from "svelte/elements";
    interface $$Props extends HTMLButtonAttributes {
        variant?: "primary" | "secondary";
        size?: "sm" | "md" | "lg";
    }

    export let variant = "primary";
    export let size = "md";
</script>

<button {...$$restProps} data-variant={variant} data-size={size}>
    <slot />
</button>

<style lang="scss">
    @use "styles/abstracts" as *;

    button {
        @include baseComponent;
        @include componentSizes;

        border-radius: 100vmin;
        border: none;

        background-color: transparent;
        color: currentColor;

        text-align: center;
        white-space: nowrap;

        transform: scale(1);
        transition: transform 100ms ease-in-out;

        &:active {
            transform: scale(0.95);
        }

        cursor: pointer;
    }

    // Variants
    button[data-variant="primary"] {
        color: $primary;
        background-color: $text;

        &:focus-visible,
        &:hover {
            outline: none;
            background-color: $accent;
        }
    }

    button[data-variant="secondary"] {
        color: $text;
        background-color: transparent;
        border: 1px solid currentColor;

        &:focus-visible,
        &:hover {
            outline: none;
            color: $accent;
        }
    }
</style>
