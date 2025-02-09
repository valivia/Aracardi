<script lang="ts">
    import type { HTMLInputAttributes } from "svelte/elements";
    interface $$Props extends Omit<HTMLInputAttributes, "size"> {
        size?: "sm" | "md" | "lg";
        label?: string;
    }

    export let size: $$Props["size"] = "md";
    export let label: $$Props["label"] = undefined;
</script>

<div class="wrapper">
    {#if label}
        <label for="">{label}</label>
    {/if}
    <input {...$$restProps} data-size={size} />
</div>

<style lang="scss">
    @use "styles/abstracts" as *;

    .wrapper {
        @include componentSizes;
        display: flex;
        flex-direction: column;
    }

    input {
        @include baseComponent;
        @include componentSizes;

        color: var(--theme-text);
        background-color: transparent;

        border: 1px solid currentColor;
        border-radius: var(--border-radius);

        font-size: 1em;

        padding-left: 0.7em;

        &:focus-visible,
        &:hover {
            outline: none;
            color: var(--theme-accent);
        }
    }
</style>
