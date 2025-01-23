<script lang="ts">
    import type { HTMLAnchorAttributes } from "svelte/elements";
    interface $$Props extends HTMLAnchorAttributes {
        variant?: "primary" | "secondary";
        size?: "sm" | "md" | "lg";
        href: string;
    }

    export let variant = "primary";
    export let size = "md";
    export let href: string;
</script>

<a
    class="button"
    {...$$restProps}
    {href}
    data-variant={variant}
    data-size={size}
>
    <slot />
</a>

<style lang="scss">
    @use "./button.scss";
</style>
