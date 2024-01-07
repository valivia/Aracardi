<script lang="ts">
    import type { HTMLInputAttributes } from "svelte/elements";
    interface $$Props extends Omit<HTMLInputAttributes, "size"> {
        size?: "sm" | "md" | "lg";
    }

    export let size = "md";
</script>

<div class="wrapper">
    <input {...$$restProps} data-size={size} />
</div>

<style lang="scss">
    @use "styles/abstracts" as *;

    .wrapper {
        @include componentSizes;
    }

    input {
        @include baseComponent;

        color: var(--theme-text);;
        background-color: transparent;

        border: 1px solid currentColor;
        border-radius: $borderRadius;

        font-size: 1em;

        padding-left: 0.7em;

        &:focus-visible,
        &:hover {
            outline: none;
            color: var(--theme-accent);;
        }
    }
</style>
