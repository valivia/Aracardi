<script lang="ts">
    import type { Snippet } from "svelte";
    import type { HTMLButtonAttributes } from "svelte/elements";
    interface Props extends HTMLButtonAttributes {
        variant?: "primary" | "secondary";
        color?: "danger" | "warning" | "default";
        children: Snippet;
    }

    let { variant = "primary", color = "default", children, ...restProps }: Props = $props();
</script>

<button class="button" {...restProps} data-variant={variant} data-color={color}>
    {@render children()}
</button>

<style lang="scss">
    @use "./button.scss" as *;
</style>
