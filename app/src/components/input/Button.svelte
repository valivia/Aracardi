<script lang="ts">
    import type { HTMLButtonAttributes } from "svelte/elements";
    interface $$Props extends HTMLButtonAttributes {
        variant?: "primary" | "secondary";
        size?: "sm" | "md" | "lg";
    }

    export let variant = "primary";
    export let size = "md";
</script>

<button class="button" {...$$restProps} data-variant={variant} data-size={size}>
    <slot />
</button>

<style lang="scss">
    @use "./button.scss" as *;
</style>
